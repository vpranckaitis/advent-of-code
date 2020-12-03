struct Data
    min::Int
    max::Int
    char::Char
    password::String
end

function read(filename)
    data = Data[]

    f = open(filename, "r")
    for ln in eachline(f)
        m = match(r"^(\d+)-(\d+) ([a-z]): (.+)$", ln)
        push!(data, Data(
            parse(Int, m.captures[1]),
            parse(Int, m.captures[2]),
            m.captures[3][1],
            m.captures[4]
        ))
    end
    close(f)

    return data
end

function solve1(input::Array{Data})
    cnt = 0
    for d in input
        char_count = 0
        for c in d.password
            if c == d.char
                char_count += 1
            end
        end

        if char_count >= d.min && char_count <= d.max
            cnt += 1
        end
    end
    return cnt
end

function solve2(input::Array{Data})
    cnt = 0
    for d in input
        if (d.password[d.min] == d.char) ⊻ (d.password[d.max] == d.char)
            cnt += 1
        end
    end
    return cnt
end

function main()
    input = read("files/02.in")
    println(solve1(input))
    println(solve2(input))
end

main()