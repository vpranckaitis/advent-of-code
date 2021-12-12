import java.lang.Integer.parseInt
import java.lang.Math.{abs, max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day10 extends App:
  val MaxHeight = 9

  val input = Source.fromFile("files/10.in")

  val data = input.getLines().map { _.toList }.toSeq

  println(part1(data))

  println(part2(data))

  input.close()


  def closing(c: Char): Char = c match
    case '(' => ')'
    case '[' => ']'
    case '{' => '}'
    case '<' => '>'


  object Opening:
    def unapply(c: Char): Option[Char] = c match
      case c if c == '(' || c == '{' || c == '[' || c == '<' => Some(c)
      case _ => None


  def part1(data: Seq[List[Char]]): Int =
    @tailrec
    def iterate(sequence: List[Char], stack: List[Char]): Option[Char] =
      (sequence, stack) match
        case (Nil, _) => None
        case (Opening(c) :: seqRest, s) => iterate(seqRest, closing(c) :: s)
        case (_ :: _, Nil) => None
        case (c :: seqRest, s :: stackRest) if c == s => iterate(seqRest, stackRest)
        case (c :: _, _) => Some(c)


    data.flatMap { iterate(_, List.empty) }.map {
      case ')' => 3
      case ']' => 57
      case '}' => 1197
      case '>' => 25137
    }.sum


  def part2(data: Seq[List[Char]]): Long =
    @tailrec
    def iterate(sequence: List[Char], stack: List[Char]): List[Char] =
      (sequence, stack) match
        case (Nil, s) => s
        case (Opening(c) :: seqRest, s) => iterate(seqRest, closing(c) :: s)
        case (_ :: _, Nil) => List.empty
        case (c :: seqRest, s :: stackRest) if c == s => iterate(seqRest, stackRest)
        case (c :: _, _) => List.empty


    def score(c: Char) = c match
      case ')' => 1
      case ']' => 2
      case '}' => 3
      case '>' => 4


    var scores = data.map { iterate(_, List.empty) }.collect {
      case l @ _ :: _ => l
    }.map {
      _.foldLeft(0L) { (acc, c) => acc * 5 + score(c) }
    }.sorted

    assert(scores.length % 2 == 1)

    scores(scores.length / 2)



