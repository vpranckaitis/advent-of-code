import java.lang.Integer.parseInt
import java.lang.Math.abs
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day16 extends App:
  val input = Source.fromFile("files/16.in")

  val hexCode = input.getLines().next()

  val bits = Map(
    '0' -> "0000",
    '1' -> "0001",
    '2' -> "0010",
    '3' -> "0011",
    '4' -> "0100",
    '5' -> "0101",
    '6' -> "0110",
    '7' -> "0111",
    '8' -> "1000",
    '9' -> "1001",
    'A' -> "1010",
    'B' -> "1011",
    'C' -> "1100",
    'D' -> "1101",
    'E' -> "1110",
    'F' -> "1111",
  )

  val binaryCode = hexCode.flatMap(bits)

  println(part1(binaryCode))
  println(part2(binaryCode))

  input.close()


  def part1(binaryCode: String): Int =
    val (parsed, rest) = parsePacket(binaryCode)

    def sumVersions(p: Packet): Int =
      p match
        case Literal(v, _, _) => v
        case Op(v, _, subPackets) => v + subPackets.map(sumVersions).sum

    sumVersions(parsed)


  def part2(binaryCode: String): Long =
    val (parsed, rest) = parsePacket(binaryCode)

    def process(p: Packet): Long =
      p match
        case Literal(_, _, value) => value
        case Op(_, 0, subPackets) => subPackets.map(process).sum
        case Op(_, 1, subPackets) => subPackets.map(process).product
        case Op(_, 2, subPackets) => subPackets.map(process).min
        case Op(_, 3, subPackets) => subPackets.map(process).max
        case Op(_, 5, lhs +: rhs +: Nil) => if process(lhs) > process(rhs) then 1 else 0
        case Op(_, 6, lhs +: rhs +: Nil) => if process(lhs) < process(rhs) then 1 else 0
        case Op(_, 7, lhs +: rhs +: Nil) => if process(lhs) == process(rhs) then 1 else 0

    process(parsed)



  trait Packet {
    val version: Int
    val id: Int
  }
  case class Literal(version: Int, id: Int, value: Long) extends Packet
  case class Op(version: Int, id: Int, subPackets: Seq[Packet]) extends Packet


  object ParseInt:
    def unapply(s: String): Option[Int] = Try { Integer.parseInt(s, 2) }.toOption


  def parsePacket(binaryCode: String): (Packet, String) =
    (binaryCode.substring(0, 3), binaryCode.substring(3, 6), binaryCode.substring(6)) match
      case (ParseInt(v), ParseInt(4), rest) =>
        val (value, rest1) = parseLiteral(0, rest)
        (Literal(v, 4, value), rest1)
      case (ParseInt(v), ParseInt(id), rest) =>
        val (subPackets, rest1) = parseSubPackets(rest)
        (Op(v, id, subPackets), rest1)


  @tailrec
  def parseLiteral(acc: Long, binaryCode: String): (Long, String) =
    (binaryCode.substring(0, 1), binaryCode.substring(1, 5), binaryCode.substring(5)) match
      case ("0", ParseInt(v), rest) =>
        (acc * 16 + v, rest)
      case ("1", ParseInt(v), rest) =>
        parseLiteral(acc * 16 + v, rest)


  def parseSubPackets(binaryCode: String): (Seq[Packet], String) =
    if binaryCode.substring(0, 1) == "0" then
      (binaryCode.substring(1, 16), binaryCode.substring(16)) match
        case (ParseInt(n), rest) =>
          val (result, _) = parsePackets(Seq.empty, rest.substring(0, n))
          val rest1 = rest.substring(n)
          (result, rest1)
    else
      (binaryCode.substring(1, 12), binaryCode.substring(12)) match
        case (ParseInt(n), rest) =>
          (0 until n).foldLeft((Seq.empty[Packet], rest)) { (acc, _) =>
            val (result, rest) = acc
            val (packet, rest1) = parsePacket(rest)
            (result :+ packet, rest1)
          }


  @tailrec
  def parsePackets(acc: Seq[Packet], binaryCode: String): (Seq[Packet], String) =
    if binaryCode.isEmpty then
      (acc, "")
    else
      val (result, rest) = parsePacket(binaryCode)
      parsePackets(acc :+ result, rest)
