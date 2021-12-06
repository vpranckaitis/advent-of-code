import java.lang.Integer.parseInt
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

val Size = 5

object day4 extends App:
  val input = Source.fromFile("files/04.in")

  val lines = input.getLines()

  val numbers = lines.next().split(",").map(parseInt).toList

  val nonEmptyLines = lines.filterNot { _.isBlank }
  val boards = nonEmptyLines.toList.map { _.trim.split("""\s+""").map(parseInt).toSeq }.sliding(Size, Size).toSeq

  println(part1(numbers, boards))

  println(part2(numbers, boards))

  input.close()


def updateNumbers(n: Int, pos: Map[Int, (Int, Int)], lines: Map[Int, Int]): (Map[Int, (Int, Int)], Map[Int, Int]) =
  val updatedLines = pos.get(n).fold(lines) { (i, j) =>
    lines.updatedWith(i) {
      case Some(v) => Some(v + 1)
      case None => Some(1)
    }.updatedWith(j + Size) {
      case Some(v) => Some(v + 1)
      case None => Some(1)
    }
  }
  (pos.removed(n), updatedLines)


def toPositions(boards: Seq[Seq[Seq[Int]]]): Seq[(Map[Int, (Int, Int)], Map[Int, Int])] =
  boards.map { rows =>
    val indices: Seq[(Int, Int, Int)] = rows.zipWithIndex.flatMap { (row, i) =>
      row.zipWithIndex.map { (v, j) => (v, i, j) }
    }
    (indices.groupMapReduce{ _._1 } { t => (t._2, t._3) } { (x, _) => x }, Map.empty[Int, Int])
  }


def part1(numbers: List[Int], boards: Seq[Seq[Seq[Int]]]): Int =
  @tailrec
  def solve(numbers: List[Int], positions: Seq[(Map[Int, (Int, Int)], Map[Int, Int])]): Int =
    val n = numbers.head
    val updated = for {
      (pos, lines) <- positions
    } yield updateNumbers(n, pos, lines)

    val res: Option[Int] = updated.find { _._2.exists { _._2 == Size } }.map { case (pos, _) =>
      pos.keys.sum * n
    }

    res match {
      case Some(v) => v
      case None => solve(numbers.tail, updated)
    }

  solve(numbers, toPositions(boards))


def part2(numbers: List[Int], boards: Seq[Seq[Seq[Int]]]): Int =
  @tailrec
  def solve(lastWin: Int, numbers: List[Int], positions: Seq[(Map[Int, (Int, Int)], Map[Int, Int])]): Int =
    numbers.headOption match {
      case None => lastWin
      case Some(n) =>
        val updated = for {
          (pos, lines) <- positions
        } yield updateNumbers(n, pos, lines)

        val (wins, nots) = updated.partition { _._2.exists { _._2 == Size } }

        val res = wins.headOption.map { case (pos, _) =>
          pos.keys.sum * n
        }

        res match {
          case Some(v) => solve(v, numbers.tail, nots)
          case None => solve(lastWin, numbers.tail, nots)
        }
    }


  solve(0, numbers, toPositions(boards))