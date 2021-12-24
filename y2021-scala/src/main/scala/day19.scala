import java.lang.Integer.parseInt
import java.lang.Math.{abs, max}
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day19 extends App:
  val input = Source.fromFile("files/19.in")

  val data: List[List[Pos]] = input.getLines().filterNot { _.isBlank }.foldLeft(List.empty[List[Pos]]) { (acc, s) =>
    s match
      case s"${ParseInt(x)},${ParseInt(y)},${ParseInt(z)}" =>
        (Pos(x, y, z) :: acc.head) :: acc.tail
      case s"--- scanner ${_} ---" =>
        List.empty :: acc
      case _ =>
        acc
  }.reverse

  println(part1(data))
  println(part2(data))

  input.close()


  def part1(data: List[List[Pos]]): Int =
    val dataOfSets = data.map { _.toSet }
    findMatches(dataOfSets.take(1).map { p => (Pos(0, 0, 0), p) }, dataOfSets.tail).toSet.flatMap { _._2 }.size


  def part2(data: List[List[Pos]]): Int =
    val dataOfSets = data.map { _.toSet }
    val scannerPositions = findMatches(dataOfSets.take(1).map { p => (Pos(0, 0, 0), p) }, dataOfSets.tail).map { _._1 }
    val distances = for
      p1 <- scannerPositions
      p2 <- scannerPositions
    yield abs(p1.x - p2.x) + abs(p1.y - p2.y) + abs(p1.z - p2.z)
    distances.max


  object ParseInt:
    def unapply(s: String): Option[Int] = Try { Integer.parseInt(s) }.toOption


  case class Pos(x: Int, y: Int, z: Int)


  @tailrec
  def findMatches(matched: List[(Pos, Set[Pos])], unmatched: List[Set[Pos]]): List[(Pos, Set[Pos])] =
    @tailrec
    def findFirstWithMatches(candidates: List[(Pos, Set[Pos])], unmatched: List[Set[Pos]]): (List[(Pos, Set[Pos])], List[Set[Pos]]) =
      candidates match
        case (_, a) :: tail =>
          val alignments = unmatched.map { b => (findMatch(4, a, b), b) }
          val matches = alignments.collect { case (Some(p), _) => p }
          if matches.isEmpty then
            findFirstWithMatches(tail, unmatched)
          else
            val newUnmatched = alignments.collect { case (None, p) => p }
            (matches, newUnmatched)
        case _ =>
          (List.empty, unmatched)


    if unmatched.isEmpty then
      matched
    else
      val (additionalMatches, newUnmatched) = findFirstWithMatches(matched, unmatched)
      findMatches(matched.prependedAll(additionalMatches), newUnmatched)



  def findMatch(n: Int, first: Set[Pos], second: Set[Pos]): Option[(Pos, Set[Pos])] =
    if n == 0 then
      None
    else
      None
        .orElse(findTranslation(first, rotateX(0, second)))
        .orElse(findTranslation(first, rotateX(1, second)))
        .orElse(findTranslation(first, rotateX(2, second)))
        .orElse(findTranslation(first, rotateX(3, second)))
        .orElse(findTranslation(first, rotateX(0, rotateY(1, second))))
        .orElse(findTranslation(first, rotateX(1, rotateY(1, second))))
        .orElse(findTranslation(first, rotateX(2, rotateY(1, second))))
        .orElse(findTranslation(first, rotateX(3, rotateY(1, second))))
        .orElse(findTranslation(first, rotateX(0, rotateY(2, second))))
        .orElse(findTranslation(first, rotateX(1, rotateY(2, second))))
        .orElse(findTranslation(first, rotateX(2, rotateY(2, second))))
        .orElse(findTranslation(first, rotateX(3, rotateY(2, second))))
        .orElse(findTranslation(first, rotateX(0, rotateY(3, second))))
        .orElse(findTranslation(first, rotateX(1, rotateY(3, second))))
        .orElse(findTranslation(first, rotateX(2, rotateY(3, second))))
        .orElse(findTranslation(first, rotateX(3, rotateY(3, second))))
        .orElse(findTranslation(first, rotateX(0, rotateZ(1, second))))
        .orElse(findTranslation(first, rotateX(1, rotateZ(1, second))))
        .orElse(findTranslation(first, rotateX(2, rotateZ(1, second))))
        .orElse(findTranslation(first, rotateX(3, rotateZ(1, second))))
        .orElse(findTranslation(first, rotateX(0, rotateZ(3, second))))
        .orElse(findTranslation(first, rotateX(1, rotateZ(3, second))))
        .orElse(findTranslation(first, rotateX(2, rotateZ(3, second))))
        .orElse(findTranslation(first, rotateX(3, rotateZ(3, second))))


  def translate(p: Pos, d: Pos) = Pos(p.x + d.x, p.y + d.y, p.z + d.z)
  def translateMany(a: Set[Pos], d: Pos) = a.map { p => translate(p, d) }


  def findTranslation(first: Set[Pos], second: Set[Pos]): Option[(Pos, Set[Pos])] =
    val offsets = for
      a <- first
      b <- second
      d = Pos(a.x - b.x, a.y - b.y, a.z - b.z)
    yield d

    offsets.find { d => second.count { p => first(translate(p, d)) } >= 12 }.map { d => (d, translateMany(second, d)) }


  def rotateX(n: Int, p: Set[Pos]) = transform(n, p) { p => Pos( p.x, -p.z,  p.y) }
  def rotateY(n: Int, p: Set[Pos]) = transform(n, p) { p => Pos( p.z,  p.y, -p.x) }
  def rotateZ(n: Int, p: Set[Pos]) = transform(n, p) { p => Pos(-p.y,  p.x,  p.z) }

  @tailrec
  def transform(n: Int, p: Set[Pos])(fn: Pos => Pos): Set[Pos] =
    if n == 0 then
      p
    else
      transform(n - 1, p.map(fn))(fn)