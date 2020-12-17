const Point = Vector{Int}

function read(filename)::Vector{Point}
    return open(filename, "r") do f
        data = Vector{Point}()
        map(enumerate(eachline(f))) do (y, ln)
            for (x, c) in enumerate(ln[:])
                c == '#' && push!(data, [x - 1, y - 1])
            end
        end
        data
    end
end

function solve(data)
    dims = length(data[1])

    adj::Vector{Point} = [zeros(dims)]
    for i in 1:dims
        adj1::Vector{Point} = adj[:]
        for p in adj
            for d in [-1 1]
                p1 = p[:]
                p1[i] = d
                push!(adj1, p1)
            end
        end
        adj = adj1
    end

    setdiff!(adj, [zeros(dims)])

    current::Set{Point} = Set{Point}(data)
    for i in 1:6
        counts::Dict{Point, Int} = Dict()
        for p in current
            for d in adj
                p1 = p + d
                counts[p1] = get(counts, p1, 0) + 1
            end
        end

        next::Set{Point} = Set()
        for (p, cnt) in counts
            (cnt == 3 || cnt == 2 && in(p, current)) && push!(next, p)
        end

        current = next
    end

    return length(current)
end

function solve1(data)
    data = map(data) do v
        vcat(v, [0])
    end
    return solve(data)
end

function solve2(data)
    data = map(data) do v
        vcat(v, [0, 0])
    end
    return solve(data)
end


function main()
    input = read("files/17.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()