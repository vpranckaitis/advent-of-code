function read(filename)::Vector{Vector{Bool}}
    return open(filename, "r") do f
        return map(eachline(f)) do ln
            return [c == 'L' for c in ln[:]]
        end
    end
end

function solve(data, min_leave, adj_fn)
    n = length(data) + 2
    m = length(data[1]) + 2
    seats = falses(n, m)

    for i in 1:length(data), j in 1:length(data[1])
        seats[i + 1, j + 1] = data[i][j]
    end

    adj = adj_fn(seats)

    seatings = zeros(n, m)
    changed = true
    while changed
        changed = false
        seatings1 = zeros(Int, n, m)
        for i in 2:(n - 1), j in 2:(m - 1)
            if seats[i, j]
                cnt = 0
                for (ii, jj) in adj[i, j]
                    cnt += seats[ii, jj] ? seatings[ii, jj] : 0
                end

                seatings1[i, j] = if seatings[i, j] == 0 && cnt == 0
                    1
                elseif seatings[i, j] == 1 && cnt >= min_leave
                    0
                else
                    seatings[i, j]
                end
            end
        end

        changed = (seatings != seatings1)
        seatings = seatings1
    end

    return sum(seatings)
end

function solve1(data)
    adj_fn = seats -> begin
        n = size(seats, 1)
        m = size(seats, 2)
        adj = Array{Vector{Tuple{Int, Int}}}(undef, n, m)
        for i in 2:(n - 1), j in 2:(m - 1)
            adj[i, j] = Vector{Tuple{Int, Int}}()
            for ii in (i - 1):(i + 1), jj in (j - 1):(j + 1)
                if ii != i || jj != j
                    push!(adj[i, j], (ii, jj))
                end
            end
        end
        return adj
    end

    return solve(data, 4, adj_fn)
end

function getnearest(i, j, di, dj, ri, rj, seats)::Tuple{Int, Int}
    if i < 1 || i > size(seats, 1) || j < 1 || j > size(seats, 2)
        return (0, 0)
    elseif ri[i, j] == 0
        t = getnearest(i + di, j + dj, di, dj, ri, rj, seats)
        ri[i, j] = t[1]
        rj[i, j] = t[2]
    end

    if seats[i, j]
        return (i, j)
    else
        return (ri[i, j], rj[i, j])
    end
end

function solve2(data)
    adj_fn = seats -> begin
        n = size(seats, 1)
        m = size(seats, 2)

        ri = Vector{Array{Int, 2}}()
        rj = Vector{Array{Int, 2}}()

        for di in -1:1, dj in -1:1
            if di != 0 || dj != 0
                push!(ri, zeros(n, m))
                push!(rj, zeros(n, m))
                for i in 1:n, j in 1:m
                    getnearest(i, j, di, dj, ri[end], rj[end], seats)
                end
            end
        end

        adj = Array{Vector{Tuple{Int, Int}}}(undef, n, m)
        for i in 1:n, j in 1:m
            adj[i, j] = Vector{Tuple{Int, Int}}()
            for k in 1:8
                if ri[k][i, j] != 0
                    push!(adj[i, j], (ri[k][i, j], rj[k][i, j]))
                end
            end
        end

        return adj
    end

    return solve(data, 5, adj_fn)
end


function main()
    input = read("files/11.in")
    println(solve1(input))
    println(solve2(input))
end

main()