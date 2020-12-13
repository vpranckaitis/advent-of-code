function read(filename)::Tuple{Int, Vector{Tuple{Int, Int}}}
    return open(filename, "r") do f
        lines = collect(eachline(f))
        t = parse(Int, lines[1])
        busses = [(parse(Int, b), i) for (i, b) in enumerate(split(lines[2], ',')) if b != "x"]
        (t, busses)
    end
end

function solve1((t, busses))
    diffs = map(busses) do b
        bid = b[1]
        tb = if t % bid == 0
            t
        else
            (t ÷ bid + 1) * bid
        end
        (tb - t, bid)
    end
    diff, id = min(diffs...)
    return diff * id
end

function solve2((_, busses))
    sorted = map(p -> (big(p[1]), big(p[1] - (p[2] - 1) % p[1])), sort(busses, rev=true))

    n1 = big(1)
    a1 = big(0)
    for (n2, a2) in sorted
        _, m1, m2 = gcdx(n1, n2)
        t = a1*m2*n2 + a2*m1*n1

        n1 = n1 * n2
        a1 = t % n1
    end

    t = (a1 + n1) % n1

    return t
end


function main()
    input = read("files/13.in")
    println(solve1(input))
    println(solve2(input))
end

main()