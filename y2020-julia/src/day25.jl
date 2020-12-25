const MOD = 20201227

function read(filename)::Vector{Int64}
    return open(filename, "r") do f
        map(eachline(f)) do ln
            parse(Int64, ln)
        end
    end
end

function solve1(data)
    subject = 7

    pubk = 1
    enck = 1
    while pubk != data[1]
        pubk = (pubk * subject) % MOD
        enck = (enck * data[2]) % MOD
    end
    return enck
end

function main()
    input = read("files/25.in")
    @time println(solve1(input))
end

main()