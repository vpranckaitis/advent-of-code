function read(filename)
    return open(filename, "r") do f
        data = Vector{Vector{String}}()

        group = Vector{String}()
        for ln in eachline(f)
            if isempty(ln)
                if !isempty(group)
                    push!(data, group)
                    group = Vector{String}()
                end
            else
                push!(group, ln)
            end
        end

        if !isempty(group)
            push!(data, group)
        end

        data
    end
end

function solve(combine::Function, groups)
    res = 0
    for g in groups
        sets = map(g) do v
            Set(v[:])
        end
        res += length(combine(sets...))
    end
    return res
end

function solve1(groups)
    return solve(union, groups)
end

function solve2(groups)
    return solve(intersect, groups)
end

function main()
    input = read("files/06.in")
    println(solve1(input))
    println(solve2(input))
end

main()