import java.lang.Integer.parseInt
import java.lang.Math.abs
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day13 extends App:
  trait Input
  trait Fold extends Input
  case class Point(x: Int, y: Int) extends Input
  case class FoldX(x: Int) extends Fold
  case class FoldY(y: Int) extends Fold

  object IsInt:
    def unapply(s: String): Option[Int] =
      Try { parseInt(s) } match
        case Success(v) => Some(v)
        case _ => None

  val input = Source.fromFile("files/13.in")

  val data: Seq[Input] = input.getLines().flatMap { l =>
    l match
      case s"${IsInt(x)},${IsInt(y)}" => Some(Point(x, y))
      case s"fold along x=${IsInt(x)}" => Some(FoldX(x))
      case s"fold along y=${IsInt(y)}" => Some(FoldY(y))
      case _ => None
  }.toSeq

  val points = data.collect { case p: Point => p }
  val folds = data.collect { case f: Fold => f }

  println(part1(points, folds))
  println(part2(points, folds))

  input.close()


  @tailrec
  def doFolds(points: Set[Point], folds: List[Fold]): Set[Point] =
    folds match
      case Nil => points
      case FoldX(fx) :: rest =>
        val updatedPoints = points.map { case p@Point(x, y) =>
          if x <= fx then p else Point(2 * fx - x, y)
        }
        doFolds(updatedPoints, rest)
      case FoldY(fy) :: rest =>
        val updatedPoints = points.map { case p@Point(x, y) =>
          if y <= fy then p else Point(x, 2 * fy - y)
        }
        doFolds(updatedPoints, rest)


  def part1(points: Seq[Point], folds: Seq[Fold]): Int =
    doFolds(points.toSet, folds.take(1).toList).size


  def part2(points: Seq[Point], folds: Seq[Fold]): String =
    val finalPoints = doFolds(points.toSet, folds.toList)

    val maxX = finalPoints.map { _.x }.max
    val maxY = finalPoints.map { _.y }.max

    (0 to maxY).map { y =>
      (0 to maxX).map { x => if finalPoints(Point(x, y)) then '#' else '.' }.mkString
    }.mkString("\n")