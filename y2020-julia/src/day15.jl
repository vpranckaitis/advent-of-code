function read(filename)::Vector{Int}
    return open(filename, "r") do f
        map(x -> parse(Int, x), split(collect(eachline(f))[1], ","))
    end
end

function solve(data, k)
    next = 0
    occur = Dict{Int, Int}()
    for i in 1:length(data)
        next = haskey(occur, data[i]) ? i - occur[data[i]] : 0
        occur[data[i]] = i
    end

    for i in (length(data) + 1):(k - 1)
        new_next = haskey(occur, next) ? i - occur[next] : 0
        occur[next] = i
        next = new_next
    end
    return next
end

function solve1(data)
    return solve(data, 2020)
end

function solve2(data)
    return solve(data, 30000000)
end


function main()
    input = read("files/15.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()