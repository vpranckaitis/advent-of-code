import java.lang.Integer.parseInt
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day3 extends App:
  val input = Source.fromFile("files/03.in")

  val bits = input.getLines().map { _.map { c => parseInt(c.toString) }.toSeq }.toSeq

  println(part1(bits))

  println(part2(bits))

  input.close()


def toInt(b: Seq[Int]): Int =
  b.foldLeft(0) { (acc, b) => (acc << 1) + b }


def part1(bits: Seq[Seq[Int]]): Int =
  val gammaBits = bits.transpose.map { b =>
    val counts = b.groupBy(identity).view.mapValues{ _.size }.toMap.withDefaultValue(0)
    if counts(0) > counts(1) then 0 else 1
  }

  val gamma = toInt(gammaBits)
  val epsilon = ((1 << gammaBits.size) - 1) ^ gamma

  gamma * epsilon


def part2(bits: Seq[Seq[Int]]): Int =
  @tailrec
  def select(b: Seq[Seq[Int]], idx: Int, f: (Int, Int) => Int): Seq[Int] =
    if b.sizeIs == 1 then
      b.head
    else
      val zeros = b.map{ _(idx) }.count{ _ == 0 }
      val ones = b.size - zeros
      val bitToFilter = f(zeros, ones)
      val b1 = b.filter { _(idx) == bitToFilter }
      select(b1, idx + 1, f)

  def f1(zeros: Int, ones: Int) = if zeros > ones then 0 else 1
  def f2(zeros: Int, ones: Int) = if ones < zeros then 1 else 0

  toInt(select(bits, 0, f1)) * toInt(select(bits, 0, f2))