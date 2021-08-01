using Printf
using Random
using JuMP
using GLPK

rs = MersenneTwister(1234)

model = Model(GLPK.Optimizer)

# number of pairs and steps
p = 4;
n = p;

# number of elements in A and B
na = 10;
nb = 10;

# cache size for a and b
ca = 1;
cb = 2;

P = Array{Tuple{Int64,Int64},1}(undef,0)
# problem inputs
for i in 1:p
	push!(P, (rand(rs, 1:na), rand(rs, 1:nb)))
end

# display the solved variables
function dispres(name, val, count1, count2, shouldtranspose)
	println(name)
	vals = value.(val)
	sol = zeros(Int, count1, count2)
	for i = 1:count1
		for j = 1:count2
			sol[i, j] = round(Int, vals[i, j])
		end
	end
	if shouldtranspose
		display(transpose(sol))
	else
		display(sol)
	end
end


# a values, b values, and whether or not the pairs are present
@variable(model, a[i=1:n, j=1:na], Bin)
@variable(model, b[i=1:n, j=1:nb], Bin)

# no elements initially loaded into the cache
for i = 1:na
	@constraint(model, a[1, i] == 0)
end
for i = 1:nb
	@constraint(model, b[1, i] == 0)
end

# there can't be more elements cached than the size of each respective cache
# (we can ignore the first column as it is entirely 0
for i in 2:n
	@constraint(model, sum(a[i, j] for j = 1:na) <= ca)
	@constraint(model, sum(b[i, j] for j = 1:nb) <= cb)
end


# define the pairs
@variable(model, pairs[i=1:n, j=1:p], Bin)

# pair definition constraints
for i in 1:n
	for pair in 1:p
		@constraints(model, begin
			pairs[i, pair] <= a[i, P[pair][1]]
			pairs[i, pair] <= b[i, P[pair][2]]
			pairs[i, pair] >= a[i, P[pair][1]] + b[i, P[pair][2]] - 1
		end)
	end
end

for pair in 1:p
	@constraint(model, sum(pairs[i, pair] for i=1:n) >= 1)
end


# the cost of caching a
@variable(model, acost[i=1:n, j=1:na], Bin)
@variable(model, bcost[i=1:n, j=1:nb], Bin)

# initial value of the cost
for i = 1:na
	@constraint(model, acost[1, i] == 0)
end
for i = 1:nb
	@constraint(model, bcost[1, i] == 0)
end
for i = 2:n
	for j = 1:na
		@constraint(model, acost[i, j] >= a[i, j] - a[i - 1, j])
		@constraint(model, acost[i, j] >= 0)
	end
	for j = 1:nb
		@constraint(model, bcost[i, j] >= b[i, j] - b[i - 1, j])
		@constraint(model, bcost[i, j] >= 0)
	end
end


@objective(model, Min, sum(acost[:, :]))# + sum(bcost[:, :]))

optimize!(model)

dispres("A in cache", a, n, na, true)
dispres("B in cache", b, n, nb, true)

#dispres("cost of a", acost, n, na, true)

for i=1:n
	#dispres(@sprintf("pairs at i=%d",i), pairs[i,:,:], na, nb)
end