import java.lang.Integer.parseInt
import java.lang.Math.{max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day06 extends App:
  val input = Source.fromFile("files/06.in")

  val timers = input.getLines().next().split(",").map(parseInt)

  println(part1(timers))

  println(part2(timers))

  input.close()


  def part1(timers: Seq[Int]) = solve(80, timers)
  def part2(timers: Seq[Int]) = solve(256, timers)


  def solve(n: Int, timers: Seq[Int]): Long =
    @tailrec
    def iterate(n: Int, times: Map[Int, Long]): Map[Int, Long] =
      if n == 0 then
        times
      else
        val updates: Seq[(Int, Long)] = times.toSeq.flatMap { (t, k) =>
          if t == 0 then
            Seq((6, k), (8, k))
          else
            Seq((t - 1, k))
        }
        val newTimes = updates.groupMapReduce { _._1 } { _._2 } { _ + _ }
        iterate(n - 1, newTimes)



    val initialTimes = timers.groupBy(identity).view.mapValues { _.size.toLong }.toMap
    val times = iterate(n, initialTimes)

    times.values.sum
