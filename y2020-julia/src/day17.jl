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
    adj::Vector{Point} = [[]]
    for i in 1:length(data[1])
        adj1::Vector{Point} = []
        for p in adj
            for d in -1:1
                p1 = push!(p[:], d)
                push!(adj1, p1)
            end
        end
        adj = adj1
    end

    adj = filter(adj) do d
        sum(abs.(d)) != 0
    end

    current = Set{Point}(data)
    for i in 1:6
        next::Set{Point} = Set()
        for p in current
            cnt = 0
            for d in adj
                in(p + d, current) && (cnt += 1)
            end
            2 <= cnt <= 3 && push!(next, p)
        end

        inactives::Dict{Point, Int} = Dict()
        for p in current
            for d in adj
                p1 = p + d
                inactives[p1] = get(inactives, p1, 0) + 1
            end
        end

        for (p, cnt) in inactives
            cnt == 3 && push!(next, p)
        end

        current = next
    end

    return length(current)
end

function solve1(data)
    data = map(data) do v
        res = Vector{Int}()
        push!(res, v...)
        push!(res, 0)
        res
    end
    return solve(data)
end

function solve2(data)
    data = map(data) do v
        res = Vector{Int}()
        push!(res, v...)
        push!(res, [0, 0]...)
        res
    end
    return solve(data)
end


function main()
    input = read("files/17.in")
    println(solve1(input))
    println(solve2(input))
end

main()