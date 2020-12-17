function read(filename)::Vector{Int64}
    return open(filename, "r") do f
        return map(ln -> parse(Int64, ln), eachline(f))
    end
end

function solve1(data)
    for i in 26:length(data)
        found = false

        for j in (i-25):(i - 1)
            for k in (j + 1):(i - 1)
                data[j] + data[k] == data[i] && (found = true)
            end
        end

        found || return data[i]
    end
end

function solve2(data, target_sum)
    i = 1
    j = 1
    sum = data[1]

    while sum != target_sum || i == j
        if sum < target_sum || i == j
            j += 1
            sum += data[j]
        else
            sum -= data[i]
            i += 1
        end
    end
    return min(data[i:j]...) + max(data[i:j]...)
end


function main()
    input = read("files/09.in")
    @time ans1 = solve1(input)
    println(ans1)
    @time println(solve2(input, ans1))
end

main()