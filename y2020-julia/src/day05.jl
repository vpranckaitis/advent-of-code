function read(filename)
    data = String[]

    f = open(filename, "r")

    for ln in eachline(f)
        push!(data, ln)
    end
    close(f)

    return data
end

function getid(s)
    id = 0
    for c in s
        id <<= 1
        if c == 'B' || c == 'R'
            id += 1
        end
    end
    return id
end

function solve1(seats)
    mx = 0
    for s in seats
        mx = max(mx, getid(s))
    end
    return mx
end

function solve2(seats)
    seat_ids = map(seats) do s
        getid(s)
    end
    sort!(seat_ids)
    for i in 2:length(seat_ids)
        if seat_ids[i - 1] + 1 != seat_ids[i]
            return seat_ids[i - 1] + 1
        end
    end
    return -1
end

function main()
    input = read("files/05.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()