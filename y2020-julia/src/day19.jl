const Rule = Union{Char, Vector{Vector{Int}}}
const RuleDict = Dict{Int, Rule}

function read(filename)::Tuple{RuleDict, Vector{String}}
    return open(filename, "r") do f
        read_rules = true
        rules = RuleDict()
        msgs = Vector()

        for ln in eachline(f)
            if isempty(ln)
                read_rules = false
            elseif read_rules
                parts = split(ln, ":")
                id = parse(Int, parts[1])
                rule = if occursin("\"", parts[2])
                    match(r".*\"(.)\".*", parts[2]).captures[1][1]
                else
                    parts = split(parts[2], "|")
                    map(parts) do p
                        map(x -> parse(Int, x), split(p, " "; keepempty=false))
                    end
                end
                rules[id] = rule
            else
                push!(msgs, ln)
            end
        end
        return (rules, msgs)
    end
end

function check(memo, rules, msg, i::Int, c::Char)::Set{Int}
     return if i <= length(msg) && msg[i] == c
        Set([i + 1])
     else
         Set()
     end
end

function check(memo, rules, msg, i::Int, sequence::Vector{Int})::Set{Int}
    from = Set([i])
    for id in sequence
        from = check(memo, rules, msg, from, id)
    end
    return from
end

function check(memo, rules, msg, i::Int, branches::Vector{Vector{Int}})::Set{Int}
    to = Set()
    for sequence in branches
        union!(to, check(memo, rules, msg, i, sequence))
    end
    return to
end

function check(memo, rules, msg, i::Int, id::Int)::Set{Int}
    return get!(memo, (i, id)) do
        check(memo, rules, msg, i, rules[id])
    end
end

function check(memo, rules, msg, is::Set{Int}, rule)::Set{Int}
    to = Set()
    for i in is
        union!(to, check(memo, rules, msg, i, rule))
    end
    return to
end

function solve1((rules, msgs))
    cnt = 0
    for msg in msgs
        memo = Dict()
        matches = check(memo, rules, msg, 1, rules[0])
        in(length(msg) + 1, matches) && (cnt += 1)
    end
    return cnt
end

function solve2((rules, msgs))
    rules = copy(rules)
    # 8: 42 | 42 8
    # 11: 42 31 | 42 11 31
    rules[8] = [[42], [42, 8]]
    rules[11] = [[42, 31], [42, 11, 31]]

    cnt = 0
    for msg in msgs
        memo = Dict()
        matches = check(memo, rules, msg, 1, rules[0])
        in(length(msg) + 1, matches) && (cnt += 1)
    end
    return cnt
end


function main()
    input = read("files/19.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()