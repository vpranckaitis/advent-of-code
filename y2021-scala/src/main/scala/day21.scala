import java.lang.Integer.parseInt
import java.lang.Math.{abs, max}
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day21 extends App:
  val input = Source.fromFile("files/21.in")

  val data = input.getLines().map {
    case s"Player ${_} starting position: ${ParseInt(p)}" => p
  }.toSeq

  println(part1(data(0), data(1)))
  println(part2(data(0), data(1)))

  input.close()


  def part1(p1: Int, p2: Int): Int =
    val (rolls, _, _, s1, _) = play1(0, p1, p2, 0, 0)
    rolls * s1


  def part2(p1: Int, p2: Int): Long =
    val (w1, w2, _) = play2(p1, p2, 0, 0, Map.empty)
    max(w1, w2)


  @tailrec
  def play1(rolls: Int, p1: Int, p2: Int, s1: Int, s2: Int): (Int, Int, Int, Int, Int) =
    if s2 >= 1000 then
      (rolls, p1, p2, s1, s2)
    else
      val rolled = rolls % 100 + (rolls + 1) % 100 + (rolls + 2) % 100 + 3
      val newP1 = (p1 + rolled - 1) % 10 + 1
      val newS1 = s1 + newP1
      play1(rolls + 3, p2, newP1, s2, newS1)


  type Memo = Map[(Int, Int, Int, Int), (Long, Long)]


  def play2(p1: Int, p2: Int, s1: Int, s2: Int, memo: Memo): (Long, Long, Memo) =
    val key = (p1, p2, s1, s2)
    if s1 >= 21 then
      (1, 0, memo)
    else if s2 >= 21 then
      (0, 1, memo)
    else if memo.contains(key) then
      val (w1, w2) = memo(key)
      (w1, w2, memo)
    else
      val rolls = for
        r1 <- 1 to 3
        r2 <- 1 to 3
        r3 <- 1 to 3
      yield r1 + r2 + r3

      val (w1, w2, updatedMemo) = rolls.foldLeft((0L, 0L, memo)) { (acc, roll) =>
        val newP1 = (p1 + roll - 1) % 10 + 1
        val newS1 = s1 + newP1
        val (w2, w1, updatedMemo) = play2(p2, newP1, s2, newS1, acc._3)
        (acc._1 + w1, acc._2 + w2, updatedMemo)
      }

      val evenMoreUpdatedMemo = updatedMemo.updated(key, (w1, w2))

      (w1, w2, evenMoreUpdatedMemo)


  object ParseInt:
    def unapply(s: String): Option[Int] = Try { Integer.parseInt(s) }.toOption