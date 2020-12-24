function read(filename)::Vector{Int}
    return open(filename, "r") do f
        ln = collect(Iterators.take(eachline(f), 1))[1]
        [parse(Int, d) for d in ln]
    end
end

function solve(first, g, n)::Vector{Int}
    mx = maximum(g)
    p = [first, 0, 0, 0]
    for _ in 1:n
        for i in 2:length(p)
            p[i] = g[p[i - 1]]
        end

        d = p[1]
        while in(d, p)
            d = d == 1 ? mx : d - 1
        end

        g[p[1]] = g[p[4]]
        g[p[4]] = g[d]
        g[d] = p[2]

        p[1] = g[p[1]]
    end

    return g
end

function solve1(data)
    g::Vector{Int} = zeros(length(data))
    for i in 1:length(data) - 1
        g[data[i]] = data[i + 1]
    end
    g[data[end]] = data[1]

    g = solve(data[1], g, 100)

    res = []
    p = g[1]
    while p != 1
        push!(res, string(p))
        p = g[p]
    end

    return *(res...)
end

function solve2(data)
    mx = 1_000_000

    g = Vector{Int}()
    append!(g, 2:mx)
    append!(g, data[1])
    for i in 1:length(data) - 1
        g[data[i]] = data[i + 1]
    end
    g[data[end]] = 10

    g = solve(data[1], g, 10_000_000)

    return g[1] * g[g[1]]
end


function main()
    input = read("files/23.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()