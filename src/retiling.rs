use crate::util::math::*;
use crate::config::*;

use glam::*;
use serde::{Serialize, Deserialize};
use std::vec::Vec;

// return, as a set of coordinates in gridspace
fn get_covered_tiles(pixel_bounds: Dabb2, tile_size: IVec2) -> Dabb2 {
    Dabb2::bounds(
        pixel_bounds.begin.floor_on_interval(tile_size),
        pixel_bounds.end.floor_on_interval(tile_size)
    ) / tile_size
}


#[derive(Serialize, Deserialize, Debug)]
struct SampleRegion {
    input_coord: IVec3,                   
    pixel_region: Dabb2
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    output_coord: IVec3,
    sample_regions: Vec<SampleRegion>
}

struct OutputAccumulator {

}

/*
fn AddSamples(dp: DatasetProvider, job: Job, output: OutputAccumulator) {
    let texel_multiple: i32 = 1 << job.output_coord.z;

    const ivec2 MyPixelBegin = *Conf.OutputCoordTexels(OutputCoord).begin();

    for (SampleRegion const& Region : Regions) {
        if (!RunningFlag) return true;

        const auto Data = reinterpret_cast<const uint16_t*>(Load(Region.InputCoord));

        if (!Data)
            continue;

        const ivec2 InputCoordTexelBegin = Conf.InputCoordTexels(Region.InputCoord).Begin;

        for (ivec2 Pixel : Region.PixelRegion) {
            ivec2 MyPixel = (Pixel + InputCoordTexelBegin - MyPixelBegin) >> OutputCoord.z;
            Pixel = Conf.InputTileSize - 1 - Pixel;
            Samples.AddSample(MyPixel, Data[Pixel.y * Conf.InputTileSize.x + Pixel.x]);
        }
    }

    return false;
}

fn GenJobs() -> Vec<Job> {
    vector<Job> Res;

    for (int Level = Conf.EndOutputLevel; Level >= Conf.BeginOutputLevel; --Level) {
        // How many texel units fit inside one texel of output
        const int TexelMultiple = 1 << Level;

        // How big the output tile is in input texels
        const ivec2 OutputSizeTexels(TexelMultiple * Conf.OutputTileSize);

        // Set of output tiles that are needed to cover the config output range
        const DiscreteAABB2<int> CoveredOutputRegion = GetCoveredTiles(Conf.OutputPixelRange, OutputSizeTexels);

        for (ivec2 const& OutCoord : CoveredOutputRegion) {
            // Generate a job description for each output tile
            Res.emplace_back();
            Job& AddedJob = Res.back();
            
            // ...
            AddedJob.OutputCoord = ivec3(OutCoord, Level);

            // The region in input texels that this output tile covers
            const DiscreteAABB2<int> OutPixelRegion = DiscreteAABB2<int>(OutCoord) * OutputSizeTexels;

            // The set of input tiles needed to cover this pixel range
            const DiscreteAABB2<int> CoveredInputRegion = GetCoveredTiles(OutPixelRegion, Conf.InputTileSize);

            // Fill in the job with all these regions
            AddedJob.Regions.reserve(CoveredInputRegion.Area());
            for (ivec2 const& InCoord : CoveredInputRegion) {
                // Texels that this input tile occupies
                const DiscreteAABB2<int> InputTexelRegion = DiscreteAABB2<int>(InCoord) * Conf.InputTileSize;

                AddedJob.Regions.emplace_back();
                SampleRegion &Region = AddedJob.Regions.back();
                
                // ...
                Region.InputCoord = InCoord;

                // Intersect the texel region of the output tile and the texel region of the input tile
                // Then shift it into texel space of the input tile
                Region.PixelRegion = (OutPixelRegion && InputTexelRegion) - *InputTexelRegion.begin();

                htAssert(!Region.PixelRegion.Empty());
                
            }
        }
    }

    return Res;
}


vector<string> ValidateConfig(Config const& Conf) {
    vector<string> errors;
    if (!(Conf.SpatialConfig.BeginOutputLevel >= 0)) errors.push_back("Can't output a level lower than 0");
    if (!(Conf.SpatialConfig.EndOutputLevel >= Conf.SpatialConfig.EndOutputLevel)) errors.push_back("End output level must not be less than begin output level");
    if (!(Conf.SpatialConfig.InputTileSize.x > 0 && Conf.SpatialConfig.InputTileSize.y > 0)) errors.push_back("Tiles must have have size greater than 0");
    
    // remove these later
    htAssert(Conf.SpatialConfig.HasNoOffset());
    htAssert(Conf.SpatialConfig.HasUnitRatio().x && Conf.SpatialConfig.HasUnitRatio().y);

    return errors;
}

bool Convert(Config const& Conf, LogStreamFunc const& StreamLog, std::atomic_bool& RunningFlag) {
    const auto Jobs = GenJobs(Conf.SpatialConfig);

    ImageSamples Samples(Conf.SpatialConfig.OutputTileSize);
    DatasetCache Cache(".htcache/", Conf.SpatialConfig.InputTileSize.x * Conf.SpatialConfig.InputTileSize.y * 2, 128, false);

    const uint64_t InFileSize = Conf.SpatialConfig.InputTileSize.x * Conf.SpatialConfig.InputTileSize.y * Conf.DatasetConfig.InputEncoding.BitDepth / 8;
    
    const bool IsFilesystemResource = Conf.DatasetConfig.InputURIFormat.IsFilesystemResource();
    // loads and caches
    LoadFunc Load = [IsFilesystemResource, &Cache, &Conf, InFileSize, StreamLog](ivec2 const& loc) -> uint8_t* {
        auto tp0 = std::chrono::system_clock::now();
        
        // Get the name of this resource... could be made a lot faster but I doubt this line will be the bottleneck
        const string Name = FormatTileString(Conf.DatasetConfig.InputURIFormat, ivec3(loc, 0));

        // Early return if its a file and the specified file doesn't exist
        if (IsFilesystemResource && !FileExists(Name)) return nullptr;

        if (Cache.IsInCache(Name)) return Cache[Name].Data;

        auto RawData = IsFilesystemResource ? ReadEntireFileBinary(Name) : ReadEntireUrlBinary(Name);

        if (RawData.empty()) return nullptr;

        auto tp1 = std::chrono::system_clock::now();

        std::cout << "Loading took " << (tp1 - tp0).count() / 1000000 << " ms\n";

        //std::cout << "Loaded resource " << Name << "\n";

        if (Conf.DatasetConfig.InputEncoding.Encoding == FormatEncoding::PNG) {
            RawData = ReadPng(RawData, false).data;
        }

        if (Conf.DatasetConfig.InputEncoding.SwapEndian) {
            htAssert(Conf.DatasetConfig.InputEncoding.BitDepth == 16);
            for (int i = 0; i < RawData.size(); i += 2)
                std::swap(RawData[i], RawData[i + 1]);
        }

        auto Data = Cache[Name];

        memcpy(Data.Data, RawData.data(), RawData.size());

        auto tp2 = std::chrono::system_clock::now();

        StreamLog(new TileLoadedItem(ivec3(loc.x, loc.y, 0), tp2 - tp1));

        return Data.Data;
    };

    vector<uint8_t> OutputData(Conf.SpatialConfig.OutputTileSize.x * Conf.SpatialConfig.OutputTileSize.y * Conf.DatasetConfig.OutputEncoding.BitDepth, 0);

    htAssert(Conf.DatasetConfig.OutputEncoding.BitDepth == 16);

    for (Job const& j : Jobs) {
        if (!RunningFlag) return true;

        std::chrono::system_clock::time_point genStart = std::chrono::system_clock::now();

        std::cout << "Processing output tile " << js::Save(j.OutputCoord).dump() << "\n";
        if (j.AddSamples(Conf.SpatialConfig, Load, Samples, RunningFlag)) {
            // Didn't finish normally
            std::cout << "Stopped during sampling, skipping tile output\n";
            return true;
        }
        std::cout << " ... " << Samples.GetTotalSamples() << " samples\n";

        if (Conf.DatasetConfig.OutputEncoding.BitDepth == 8) {
            Samples.GenerateData<uint8_t>(reinterpret_cast<uint8_t*>(OutputData.data()), 0);
        } else if (Conf.DatasetConfig.OutputEncoding.BitDepth == 16) {
            Samples.GenerateData<uint16_t>(reinterpret_cast<uint16_t*>(OutputData.data()), 0);
        } else if (Conf.DatasetConfig.OutputEncoding.BitDepth == 32) {
            Samples.GenerateData<uint32_t>(reinterpret_cast<uint32_t*>(OutputData.data()), 0);
        } else if (Conf.DatasetConfig.OutputEncoding.BitDepth == 64) {
            Samples.GenerateData<uint64_t>(reinterpret_cast<uint64_t*>(OutputData.data()), 0);
        }

        vector<uint8_t> FinalOutput;

        std::chrono::system_clock::time_point genEnd = std::chrono::system_clock::now();

        // This will work... because currently, only 16 bit single channel PNG output is supported
        // But in the future more output modes will need to be supported
        WritePng(FinalOutput, OutputData.data(), Conf.SpatialConfig.OutputTileSize.x, Conf.SpatialConfig.OutputTileSize.y, Conf.DatasetConfig.OutputEncoding.SwapEndian);
        WriteEntireFileBinary(FormatTileString(Conf.DatasetConfig.OutputURIFormat, j.OutputCoord), FinalOutput);
        Samples.Clear();

        std::chrono::system_clock::time_point saveEnd = std::chrono::system_clock::now();

        StreamLog(new TileSavedItem(j.OutputCoord, genEnd - genStart, saveEnd - genEnd));
    }

    return true;
}
*/