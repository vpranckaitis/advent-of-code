function read(filename)::Vector{Tuple{Vector{String}, Vector{String}}}
    return open(filename, "r") do f
        map(eachline(f)) do ln
            m = match(r"(.*)\(contains (.*)\)", ln)
            lists = map(m.captures) do l
                split(l, ' '; keepempty = false)
            end
            (split(m.captures[1], " "; keepempty = false), split(m.captures[2], ", "; keepempty = false))
        end
    end
end

function possible_allergens(data)
    intersection = Dict()
    for (ingredients, allergens) in data, a in allergens
        if haskey(intersection, a)
            intersect!(intersection[a], ingredients)
        else
            intersection[a] = copy(ingredients)
        end
    end
    return intersection
end

function solve1(data)
    intersection = possible_allergens(data)
    count = Dict()
    for (ingredients, _) in data, ingr in ingredients
        count[ingr] = get(count, ingr, 0) + 1
    end

    no_allergens = collect(keys(count))
    for ingredients in values(intersection)
        setdiff!(no_allergens, ingredients)
    end

    return sum([count[v] for v in no_allergens])
end

function solve2(data)
    intersection = possible_allergens(data)

    matchings = Vector()
    changed = true
    while changed
        matched = []
        for (a, ingrs) in intersection
            if length(ingrs) == 1
                push!(matchings, (a, ingrs[1]))
                push!(matched, ingrs[1])
            end
        end

        for (a, ingrs) in intersection
            setdiff!(ingrs, matched)
        end
        changed = !isempty(matched)
    end

    sort!(matchings)

    return join([v for (_, v) in matchings], ",")
end


function main()
    input = read("files/21.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()