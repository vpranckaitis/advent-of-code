function read(filename)
    data = Dict{String, String}[]

    f = open(filename, "r")

    pass = Dict{String, String}()
    for ln in eachline(f)
        if isempty(ln)
            if !isempty(pass)
                push!(data, pass)
                pass = Dict{String, String}()
            end
        else
            parts = map((s) -> strip(s), split(ln, " "))
            for p in parts
                kv = split(p, ":")
                pass[kv[1]] = kv[2]
            end
        end
    end
    close(f)

    if !isempty(pass)
        push!(data, pass)
    end

    return data
end

function solve1(passports)
    keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    cnt = 0

    for passport in passports
        contains_all = true
        for k in keys
            contains_all &= haskey(passport, k)
        end
        if contains_all
            cnt += 1
        end
    end

    return cnt
end

function is_valid(key, value)
    function is_year_between(v, min, max)
        if match(r"^\d{4}$", v) !== nothing
           x = parse(Int, v)
           return min <= x <= max
        end
        return false
    end

    if key == "byr"
        return is_year_between(value, 1920, 2002)
    elseif key == "iyr"
        return is_year_between(value, 2010, 2020)
    elseif key == "eyr"
        return is_year_between(value, 2020, 2030)
    elseif key == "hgt"
        m = match(r"^(\d{2,3})(in|cm)$", value)
        if m !== nothing
            h = parse(Int, m.captures[1])
            return (m.captures[2] == "in" && 59 <= h <= 76) ||
                    (m.captures[2] == "cm" && 150 <= h <= 193)
        end
    elseif key == "hcl"
        return match(r"^#[0-9a-f]{6}$", value) !== nothing
    elseif key == "ecl"
        return in(value, ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
    elseif key == "pid"
        return match(r"^\d{9}$", value) !== nothing
    end
    return false
end

function solve2(passports)
    keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    cnt = 0

    for passport in passports
        contains_all = true
        for k in keys
            contains_all &= haskey(passport, k) && is_valid(k, passport[k])
        end
        if contains_all
            cnt += 1
        end
    end

    return cnt
end

function main()
    input = read("files/04.in")
    @time println(solve1(input))
    @time println(solve2(input))
end

main()