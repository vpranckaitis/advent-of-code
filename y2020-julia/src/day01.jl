function read(filename)
    data = Int[]

    f = open(filename, "r")
    for ln in eachline(f)
        push!(data, parse(Int, ln))
    end
    close(f)

    return data
end

function solve1(input)
    n = length(input)
    for i in 1:n
        for j in (i + 1):n
            if input[i] + input[j] == 2020
                return input[i] * input[j]
            end
        end
    end
end

function solve2(input)
    n = length(input)
    for i in 1:n
        for j in (i + 1):n
            for k in (j + 1):n
                if input[i] + input[j] + input[k] == 2020
                    return input[i] * input[j] * input[k]
                end
            end
        end
    end
end

function main()
    input = read("files/01.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()