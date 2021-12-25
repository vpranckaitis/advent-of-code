import java.lang.Integer.parseInt
import java.lang.Math.{min, max}
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day24 extends App:
  val input = Source.fromFile("files/24.in")

  val data = input.getLines().map {
    case s"inp $v" => Input(v)
    case s"${ParseOp(fn)} $v ${ParseInt(value)}" => ValueOp(v, value, fn)
    case s"${ParseOp(fn)} $v1 $v2" => VariableOp(v1, v2, fn)
  }

  val (minValue, maxValue) = part12(data.toSeq)
  println(maxValue)
  println(minValue)

  input.close()


  def part12(instructions: Seq[Instruction]): (Long, Long) =
    val groups = instructions.foldLeft(Seq.empty[Seq[Instruction]]) { (acc, ins) =>
      ins match
        case ins: Input =>
          Seq() +: acc
        case ins: _ =>
          (acc.head :+ ins) +: acc.tail
    }.reverse

    solve(groups)


  def solve(groups: Seq[Seq[Instruction]]): (Long, Long) =
    val result = groups.foldLeft(Map(0L -> (0L, 0L))) { (acc, group) =>

      acc.foldLeft(Map.empty[Long, (Long, Long)]){ (acc, kv) =>
        val (previousZ, (minPrefix, maxPrefix)) = kv
        (1 to 9).foldLeft(acc) { (acc, w) =>
          val newMinPrefix = minPrefix * 10 + w
          val newMaxPrefix = maxPrefix * 10 + w
          acc.updatedWith(applyGroup(group, w, previousZ)) {
            _ match
              case None => Some((newMinPrefix, newMaxPrefix))
              case Some((minP, maxP)) => Some((min(newMinPrefix, minP), max(newMaxPrefix, maxP)))
          }
        }
      }
    }

    result(0L)


  def applyGroup(group: Seq[Instruction], w: Int, z: Long): Long =
    val state = Map("w" -> w.toLong, "z" -> z).withDefaultValue(0L)
    applyInstructions(group, state)("z")


  def applyInstructions(instructions: Seq[Instruction], init: Map[String, Long]): Map[String, Long] =
    instructions.foldLeft(init) { (acc, ins) =>
      ins match {
        case ValueOp(var1, value, fn) =>
          acc.updated(var1, fn(acc(var1), value))
        case VariableOp(var1, var2, fn) =>
          acc.updated(var1, fn(acc(var1), acc(var2)))
      }
    }

  trait Instruction
  case class Input(var1: String) extends Instruction
  case class ValueOp(var1: String, value: Int, op: (Long, Long) => Long) extends Instruction
  case class VariableOp(var1: String, var2: String, op: (Long, Long) => Long) extends Instruction

  object ParseInt:
    def unapply(s: String): Option[Int] = Try { Integer.parseInt(s) }.toOption

  object ParseOp:
    def unapply(s: String): Option[(Long, Long) => Long] =
      s match
        case "add" => Some((x: Long, y: Long) => x + y)
        case "mul" => Some((x: Long, y: Long) => x * y)
        case "div" => Some((x: Long, y: Long) => x / y)
        case "mod" => Some((x: Long, y: Long) => x % y)
        case "eql" => Some((x: Long, y: Long) => if x == y then 1 else 0)
        case _ => None