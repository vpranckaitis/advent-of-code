import java.lang.Math.{max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day20 extends App:
  val input = Source.fromFile("files/20.in")

  val lines = input.getLines()
  val algo = lines.next()
  lines.next()
  val image = lines.map{ _.toSeq }.toSeq

  println(part1(algo, image))
  println(part2(algo, image))

  input.close()


  def part1(algo: String, image: Seq[Seq[Char]]): Int =
    val iterations = 2
    val margin = iterations * 2
    transform(iterations, algo, expand(image, margin)).map { _.count { _ == '#' } }.sum


  def part2(algo: String, image: Seq[Seq[Char]]): Int =
    val iterations = 50
    val margin = iterations * 2
    transform(iterations, algo, expand(image, margin)).map { _.count { _ == '#' } }.sum


  def expand(image: Seq[Seq[Char]], margin: Int): Seq[Seq[Char]] =
    val expandedHorizontally = image.map { row =>
      Seq.fill(margin)('.') ++: row.toSeq :++ Seq.fill(margin)('.')
    }
    val m = expandedHorizontally.head.size
    Seq.fill(margin)(()).map { _ => Seq.fill(m)('.') } ++: expandedHorizontally :++ Seq.fill(margin)(()).map { _ => Seq.fill(m)('.') }


  @tailrec
  def transform(k: Int, algo: String, image: Seq[Seq[Char]]): Seq[Seq[Char]] =
    if k == 0 then
      image
    else
      val n = image.length
      val m = image.head.length

      val updatedImage = (0 until n).map { i =>
        (0 until m).map { j =>
          val bits = for
            di <- -1 to 1
            dj <- -1 to 1
            ii = i + di
            jj = j + dj
          yield if image(min(max(ii, 0), n - 1))(min(max(jj, 0), m - 1)) == '#' then 1 else 0
          val idx = bits.foldLeft(0) { (acc, b) => (acc << 1) + b }
          algo(idx)
        }
      }

      transform(k - 1, algo, updatedImage)