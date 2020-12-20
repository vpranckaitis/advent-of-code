const Tile = Array{Bool, 2}
const N = 10

function read(filename)::Dict{Int, Tile}
    return open(filename, "r") do f
        id = 0
        i = 0
        m = falses(N, N)
        res = Dict{Int, Tile}()
        for ln in eachline(f)
            if startswith(ln, "Tile")
                id = parse(Int, match(r"Tile (\d+):", ln).captures[1])
            elseif isempty(ln)
                res[id] = m
                id = 0
                m = falses(N, N)
                i = 0
            else
                i += 1
                m[i, :] = collect(ln) .== '#'
            end
        end
        id != 0 && (res[id] = m)
        res
    end
end

function makeorientations(tile::Tile)::Vector{Tile}
    res = []
    for i in 1:2
        for j in 1:4
            push!(res, tile)
            tile = rotate(tile)
        end
        tile = tile[end:-1:begin, :]
    end

    return res
end

function rotate(tile)
    n = size(tile, 1)
    m = size(tile, 2)
    d = falses(m, n)

    for i in 1:size(tile, 1)
        d[:, n + 1 - i] = tile[i, :]
    end

    d
end

function makeadjacent(data)
    orientations::Dict{Int, Vector{Tile}} = Dict()
    for (id, tile) in data
        orientations[id] = makeorientations(tile)
    end

    bits = bs -> sum([(1 << (N - i + 1)) * b for (i, b) in enumerate(bs)])
    orientation_bits::Dict{Int, Vector{Vector{Int}}} = Dict()
    for id in keys(orientations)
        sides = Vector()
        for t in orientations[id]
            u = bits(t[begin, :])
            d = bits(t[end, :])
            l = bits(t[:, begin])
            r = bits(t[:, end])
            push!(sides, [u, r, d, l])
        end
        orientation_bits[id] = sides
    end

    opposite = [3, 4, 1, 2]

    adj = Dict{Tuple{Int, Int}, Vector{Vector{Tuple{Int, Int}}}}()

    for (id1, orients1) in orientation_bits
        for o1 in eachindex(orients1)
            adj[(id1, o1)] = map(_ -> Vector{Tuple{Int, Int}}(), opposite)
        end
        for (id2, orients2) in orientation_bits
            id1 == id2 && continue
            for o1 in eachindex(orients1), o2 in eachindex(orients2)
                for i in eachindex(opposite)
                    j = opposite[i]
                    if orientation_bits[id1][o1][i] == orientation_bits[id2][o2][j]
                        push!(adj[(id1, o1)][i], (id2, o2))
                    end
                end
            end
        end
    end

    return orientations, adj
end

function solve1(adj)
    corners = Set()
    for ((id, _), a) in adj
        isempty(a[1]) && isempty(a[2]) && push!(corners, id)
    end
    return *(corners...)
end

function solve2(orientations, adj)
    corner = missing
    for (id_orient, a) in adj
        if isempty(a[1]) && isempty(a[4])
            corner = id_orient
            break
        end
    end

    k = convert(Int, sqrt(length(orientations)))
    tiling = Array{Tuple{Int, Int}}(undef, k, k)
    tiling[1, 1] = corner
    for i in 2:k
        tiling[i, 1] = adj[tiling[i - 1, 1]][3][1]
    end
    for i in 1:k
        for j in 2:k
            tiling[i, j] = adj[tiling[i, j - 1]][2][1]
        end
    end

    n = N - 2

    joined::Array{Bool, 2} = falses(k * n, k * n)
    for i in 1:k, j in 1:k
        ii = (i - 1) * n
        jj = (j - 1) * n
        id, orient = tiling[i, j]
        joined[(ii + 1):(ii + n), (jj + 1):(jj + n)] = orientations[id][orient][2:(n + 1), 2:(n + 1)]
    end

    pattern::Array{Bool, 2} = [
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0;
        1 0 0 0 0 1 1 0 0 0 0 1 1 0 0 0 0 1 1 1;
        0 1 0 0 1 0 0 1 0 0 1 0 0 1 0 0 1 0 0 0;
    ]

    for p in makeorientations(pattern)
        matches = 0
        for i in 1:size(joined, 1), j in 1:size(joined, 2)
            matching = true
            for ii in 1:size(p, 1), jj in 1:size(p, 2)
                if i + ii > size(joined, 1) || j + jj > size(joined, 2) ||
                    (p[ii, jj] && !joined[i + ii - 1, j + jj - 1])
                    matching = false
                    break
                end
            end
            matching && (matches += 1)
        end

        if matches > 0
            return sum(joined) - sum(pattern) * matches
        end
    end

    return -1
end

function main()
    input = read("files/20.in")
    @time orientations, adj = makeadjacent(input)
    @time println(solve1(adj))
    @time println(solve2(orientations, adj))
end

main()