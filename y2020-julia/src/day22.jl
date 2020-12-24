import Pkg
Pkg.add("DataStructures")

using DataStructures

const Card = UInt64

function read(filename)::Tuple{Vector{Card}, Vector{Card}}
    return open(filename, "r") do f
        res = ([], [])
        idx = 0
        for ln in eachline(f)
            if startswith(ln, "Player")
                idx += 1
            elseif !isempty(ln)
                push!(res[idx], parse(UInt64, ln))
            end
        end
        res
    end
end

function enqueue!(q::Queue{T}, vals) where {T}
    for v in vals
        DataStructures.enqueue!(q, v)
    end
end

score(q) = sum([i * v for (i, v) in enumerate(Iterators.reverse(q))])

function solve1(data)
    q1, q2 = (Queue{Card}(), Queue{Card}())
    enqueue!(q1, data[1])
    enqueue!(q2, data[2])

    while !isempty(q1) && !isempty(q2)
        c1 = dequeue!(q1)
        c2 = dequeue!(q2)

        if c1 > c2
            enqueue!(q1, [c1, c2])
        else
            enqueue!(q2, [c2, c1])
        end
    end

    q = !isempty(q1) ? q1 : q2

    return score(q)
end

function playgame(q1, q2, factors)::Tuple{Int, UInt64}
    hashes = Set{Tuple{UInt64, UInt64}}()

    M::UInt64 = 1e16 + 9

    hashcards = q -> sum([factors[i] * c for (i, c) in enumerate(Iterators.reverse(q))])
    dequeuedhash = (h, q, c) -> h - c * factors[length(q) + 1]
    enqueuedhash = (h, c1, c2) -> h * factors[3] + c1 * factors[2] + c2 * factors[1]

    h1 = hashcards(q1)
    h2 = hashcards(q2)

    while !isempty(q1) && !isempty(q2)
        in((h1, h2), hashes) && return (1, score(q1))

        push!(hashes, (h1, h2))

        c1 = dequeue!(q1)
        c2 = dequeue!(q2)

        h1 = dequeuedhash(h1, q1, c1)
        h2 = dequeuedhash(h2, q2, c2)

        winner = if c1 <= length(q1) && c2 <= length(q2)
            q1t = Queue{Card}()
            q2t = Queue{Card}()
            enqueue!(q1t, Iterators.take(q1, c1))
            enqueue!(q2t, Iterators.take(q2, c2))

            winner, _ = playgame(q1t, q2t, factors)
            winner
        elseif c1 > c2
            1
        else
            2
        end

        if winner == 1
            enqueue!(q1, [c1, c2])
            h1 = enqueuedhash(h1, c1, c2)
        else
            enqueue!(q2, [c2, c1])
            h2 = enqueuedhash(h2, c2, c1)
        end
    end

    winner, q = !isempty(q1) ? (1, q1) : (2, q2)
    return (winner, score(q))
end

function solve2(data)
    q1 = Queue{Card}()
    q2 = Queue{Card}()
    enqueue!(q1, data[1])
    enqueue!(q2, data[2])

    factors::Vector{UInt64} = Vector()
    f::UInt64 = 1
    for i in 1:55
        push!(factors, f)
        f *= 37
    end
    return playgame(q1, q2, factors)[2]
end


function main()
    input = read("files/22.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()