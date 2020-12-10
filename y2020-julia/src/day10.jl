function read(filename)::Vector{Int64}
    return open(filename, "r") do f
        return map(ln -> parse(Int64, ln), eachline(f))
    end
end

function solve1(data)
    sorted = [0]
    push!(sorted, data...)
    sort!(sorted)
    push!(sorted, sorted[end] + 3)
    diff = [0, 0, 0]
    for i in 1:(length(sorted) - 1)
        diff[sorted[i + 1] - sorted[i]] += 1
    end
    return diff[1] * diff[3]
end

function solve2(data)
    sorted = [0]
    push!(sorted, data...)
    sort!(sorted)
    push!(sorted, sorted[end] + 3)
    dp = map(_ -> 0::Int64, sorted)
    dp[1] = 1
    for i in 2:length(sorted)
        j = i - 1
        while j >= 1 && sorted[i] - sorted[j] <= 3
            dp[i] += dp[j]
            j -= 1
        end
    end
    return dp[end]
end


function main()
    input = read("files/10.in")
    println(solve1(input))
    println(solve2(input))
end

main()