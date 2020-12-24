import Base.+

const Point = Tuple{Int, Int}

function read(filename)::Vector{Vector{String}}
    return open(filename, "r") do f
        map(eachline(f)) do ln
            dirs = []
            i = 1
            while i <= length(ln)
                if in(ln[i], ['e', 'w'])
                    push!(dirs, ln[i:i])
                    i += 1
                else
                    push!(dirs, ln[i:i+1])
                    i += 2
                end
            end
            dirs
        end
    end
end

const dirs = Dict(
    "e" => (2, 0),
    "w" => (-2, 0),
    "ne" => (1, 1),
    "nw" => (-1, 1),
    "se" => (1, -1),
    "sw" => (-1, -1),
)

+(t1::Point, t2::Point)::Point = (t1[1] + t2[1], t1[2] + t2[2])

function applyflips(data)::Set{Point}
    tiles = Dict{Point, Bool}()
    for ds in data
        p = (0, 0)
        for d in ds
            p += dirs[d]
        end
        tiles[p] = !get(tiles, p, false)
    end

    return Set(filter(p -> tiles[p], keys(tiles)))
end

function solve1(data)
    tiles = applyflips(data)
    return length(tiles)
end

function solve2(data)
    tiles = applyflips(data)

    for _ in 1:100
        counts = Dict{Point, Int}()
        for p in tiles, d in values(dirs)
            p1 = p + d
            counts[p1] = get(counts, p1, 0) + 1
        end

        new_tiles = Set()

        for (p, cnt) in counts
            !in(p, tiles) && cnt == 2 && push!(new_tiles, p)
        end

        for p in tiles
            cnt = get(counts, p, 0)
            0 < cnt <= 2 && push!(new_tiles, p)
        end
        tiles = new_tiles
    end
    return length(tiles)
end


function main()
    input = read("files/24.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()