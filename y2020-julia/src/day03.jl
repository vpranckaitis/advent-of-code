function read(filename)
    data = falses(1000, 100)

    f = open(filename, "r")
    n = 0
    m = 0
    for ln in eachline(f)
        n += 1
        m = length(ln)
        for j in 1:m
            data[n, j] = ln[j] == '#'
        end
    end
    close(f)

    data2 = falses(n, m)
    copyto!(data2, CartesianIndices((1:n, 1:m)), data, CartesianIndices((1:n, 1:m)))

    return data2
end

function solve1(input)
    n = size(input, 1)
    m = size(input, 2)

    cnt = 0

    j = 1
    for i in 2:n
        j = mod(j + 2, m) + 1
        if input[i, j]
            cnt += 1
        end
    end

    return cnt
end

function solve2(input)
    n = size(input, 1)
    m = size(input, 2)

    res = 1
    for dij in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        di, dj = dij

        cnt = 0
        j = 1
        for i in (1 +di):di:n
            j = (j + dj - 1) % m + 1
            if input[i, j]
                cnt += 1
            end
        end
        res *= cnt
    end

    return res
end

function main()
    input = read("files/03.in")
    println(solve1(input))
    println(solve2(input))
end

main()