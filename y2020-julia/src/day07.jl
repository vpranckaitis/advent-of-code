using DataStructures

struct Bag
    color::String
end

struct Container
    container::Bag
    contents::Vector{Tuple{Bag, Int}}
end

function read(filename)::Vector{Container}
    return open(filename, "r") do f
        data = Vector{Container}()

        for ln in eachline(f)
            contains = split(ln, " contain ")
            container = Bag(match(r"([\w\s]+) bags", contains[1]).captures[1])
            contents = Vector{Tuple{Bag, Int}}()
            if contains[2] != "no other bags."
                for b in split(contains[2], ", ")
                    m = match(r"(\d+) ([\w\s]+) bags?\.?", b)
                    push!(contents, (Bag(m.captures[2]), parse(Int, m.captures[1])))
                end
            end
            push!(data, Container(container, contents))
        end

        data
    end
end

function solve1(bags)
    containers = Dict{Bag, Vector{Bag}}()
    for b1 in bags
        for (b2, _) in b1.contents
            parent = get!(() -> Vector{Bag}(), containers, b2)
            push!(parent, b1.container)
        end
    end

    start = Bag("shiny gold")
    visited = Set{Bag}()
    q = Queue{Bag}()
    push!(visited, start)
    enqueue!(q, start)

    while !isempty(q)
        u = dequeue!(q)
        for v in get(() -> Vector{Bag}(), containers, u)
            if !in(v, visited)
                enqueue!(q, v)
                push!(visited, v)
            end
        end
    end

    return length(visited) - 1
end

function solve2(bags)
    contents = Dict{Bag, Vector{Tuple{Bag, Int}}}()
    for b1 in bags
        contents[b1.container] = b1.contents
    end

    start = Bag("shiny gold")
    counts = Dict{Bag, Int}()

    dfs(start, contents, counts)

    return counts[start] - 1
end

function dfs(u, contents::Dict{Bag, Vector{Tuple{Bag, Int}}}, counts::Dict{Bag, Int})::Int
    return get!(counts, u) do
        count = 0
        for (v, n) in get(() -> Vector{Bag}(), contents, u)
            count += n * dfs(v, contents, counts)
        end
        return count + 1
    end
end

function main()
    input = read("files/07.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()