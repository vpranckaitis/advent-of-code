import java.lang.Integer.parseInt
import java.lang.Math.{abs, max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day09 extends App:
  val MaxHeight = 9

  val input = Source.fromFile("files/09.in")

  val data = input.getLines().map { _.split("").map(parseInt).toSeq }.toSeq

  println(part1(data))

  println(part2(data))

  input.close()


  def expand(data: Seq[Seq[Int]]) =
    val EdgeHeight = MaxHeight
    val expandedHorizontally = data.map { EdgeHeight +: _ :+ EdgeHeight }
    val m = expandedHorizontally.head.length
    Seq.fill(m)(EdgeHeight) +: expandedHorizontally :+ Seq.fill(m)(EdgeHeight)


  def part1(data: Seq[Seq[Int]]): Int =
    val expanded = expand(data)
    val n = expanded.size
    val m = expanded.head.size

    val isLower = for
      i <- 1 until n - 1
      j <- 1 until m - 1
      di <- -1 to 1
      dj <- -1 to 1
      if abs(di) + abs(dj) == 1
    yield (i, j, expanded(i)(j) < expanded(i + di)(j + dj))

    val isLowerThanNeighbors = isLower.groupMapReduce { p => (p._1, p._2) } { _._3 } { _ && _ }
    val lowPointHeights = isLowerThanNeighbors.filter { _._2 }.keys.toSeq.map { p => expanded(p._1)(p._2) }

    lowPointHeights.sum + lowPointHeights.size


  def part2(data: Seq[Seq[Int]]): Int =
    type DSU = Map[(Int, Int), (Int, Int)]

    @tailrec
    def parent(i: Int, j: Int, dsu: DSU): (Int, Int) =
      dsu.get((i, j)) match
        case Some((ii, jj)) => parent(ii, jj, dsu)
        case None => (i, j)


    @tailrec
    def solve(pos: List[(Int, Int)], di: Int, dj: Int, grid: Seq[Seq[Int]], dsu: DSU): DSU =
      pos match
        case (i, j) :: rest =>
          val updatedDsu = if grid(i)(j) < MaxHeight && grid(i + di)(j + dj) < MaxHeight then
            val p1 = parent(i, j, dsu)
            val p2 = parent(i + di, j + dj, dsu)
            if p1 != p2 then
              dsu.updated(p1, p2)
            else
              dsu
          else
            dsu

          solve(rest, di, dj, grid, updatedDsu)
        case _ =>
          dsu


    val expanded = expand(data)
    val n = expanded.size
    val m = expanded.head.size

    val pos = for {
      i <- 1 until n - 1
      j <- 1 until m - 1
    } yield (i, j)

    val partialDsu = solve(pos.toList, 1, 0, expanded, Map.empty)
    val dsu = solve(pos.toList, 0, 1, expanded, partialDsu)

    val sizes = pos.map { p => parent(p._1, p._2, dsu) }.groupBy(identity).values.map { _.size }
    sizes.toSeq.sorted(Ordering.Int.reverse).take(3).product