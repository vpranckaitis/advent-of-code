struct Add
end

struct Mult
end

const Op = Union{Add, Mult}

struct Expr
    op::Op
    lhs::Union{Expr, Int}
    rhs::Union{Expr, Int}
end

function read(filename)::Vector{String}
    return open(filename, "r") do f
        collect(eachline(f))
    end
end

function combine_equal(ops::Vector{Op}, exprs::Vector{Union{Expr, Int}})::Expr
    ops = reverse(ops)
    exprs = reverse(exprs)

    while !isempty(ops)
        op = pop!(ops)
        lhs = pop!(exprs)
        rhs = pop!(exprs)
        push!(exprs, Expr(op, lhs, rhs))
    end

    return exprs[1]
end

function combine_addfirst(ops::Vector{Op}, exprs::Vector{Union{Expr, Int}})::Expr
    ops1::Vector{Op} = Vector()
    exprs1::Vector{Union{Expr, Int}} = Vector()

    push!(exprs1, exprs[1])

    for i in 1:length(ops)
        op = ops[i]
        if op == Add()
            lhs = pop!(exprs1)
            rhs = exprs[i + 1]
            push!(exprs1, Expr(op, lhs, rhs))
        else
            push!(ops1, op)
            push!(exprs1, exprs[i + 1])
        end
    end

    return combine_equal(ops1, exprs1)
end

function parseexpr(p::String, i::Int, combine::Function)::Tuple{Expr, Int}
    ops::Vector{Op} = Vector()
    exprs::Vector{Union{Expr, Int}} = Vector()

    while i <= length(p) && p[i] != ')'
        if p[i] == '*'
            push!(ops, Mult())
        elseif p[i] == '+'
            push!(ops, Add())
        elseif p[i] == '('
            e, k = parseexpr(p, i + 1, combine)
            push!(exprs, e)
            i = k
        elseif p[i] != ' '
            val = parse(Int, p[i:i])
            push!(exprs, val)
        end
        i += 1
    end

    return (combine(ops, exprs), i)
end

eval(expr::Expr)::Int = eval(expr.op, expr.lhs, expr.rhs)
eval(val::Int)::Int = val
eval(op::Add, lhs::Union{Expr, Int}, rhs::Union{Expr, Int})::Int = eval(lhs) + eval(rhs)
eval(op::Mult, lhs::Union{Expr, Int}, rhs::Union{Expr, Int})::Int = eval(lhs) * eval(rhs)

function solve1(data)
    return sum(map(data) do d
        eval(parseexpr(d, 1, combine_equal)[1])
    end)
end

function solve2(data)
    return sum(map(data) do d
        eval(parseexpr(d, 1, combine_addfirst)[1])
    end)
end


function main()
    input = read("files/18.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()