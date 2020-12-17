struct Range
    r1::UnitRange{Int}
    r2::UnitRange{Int}
end

function read(filename)
    return open(filename, "r") do f
        ranges = Dict{String, Range}()
        ticket = Vector{Int}()
        nearby = Vector{Vector{Int}}()
        stage = 0
        for ln in eachline(f)
            isempty(ln) && continue
            if stage == 0
                if ln == "your ticket:"
                    stage = 1
                else
                    m = match(r"^([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)$", ln)
                    name = m.captures[1]
                    r = map(x -> parse(Int, x), m.captures[2:end])
                    ranges[name] = Range(r[1]:r[2], r[3]:r[4])
                end
            else
                if ln == "nearby tickets:"
                    stage = 2
                else
                    t = map(x -> parse(Int, x), split(ln, ","))
                    if stage == 1
                        ticket = t
                    else
                        push!(nearby, t)
                    end
                end
            end
        end
        (ranges, ticket, nearby)
    end
end

function invalidvalues(ranges, values)
    return filter(values) do x
        good = false
        for r in ranges
            good |= in(x, r.r1) || in(x, r.r2)
        end
        !good
    end
end

function solve1((ranges, _, nearby))
    res = 0

    for n in nearby
        res += sum(invalidvalues(values(ranges), n))
    end
    return res
end

function solve2((ranges, ticket, nearby))
    nearby = filter(n -> isempty(invalidvalues(values(ranges), n)), nearby)

    columns = Vector{Vector{String}}()
    for i in 1:length(ticket)
        push!(columns, Vector())
        values = map(n -> n[i], nearby)
        push!(values, ticket[i])
        for (k, r) in ranges
            if isempty(invalidvalues([r], values))
                push!(columns[i], k)
            end
        end
    end

    not_processed = collect(1:length(ticket))
    while !isempty(not_processed)
        for i in not_processed
            if length(columns[i]) == 1
                c = columns[i][1]
                for j in not_processed
                    i != j && setdiff!(columns[j], [c])
                end
                setdiff!(not_processed, i)
            end
        end
    end

    res_column_idxs = filter(i -> startswith(columns[i][1], "departure"), 1:length(ticket))
    return *(ticket[res_column_idxs]...)
end


function main()
    input = read("files/16.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()