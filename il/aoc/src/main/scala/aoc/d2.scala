package aoc
import scala.io.Source

object AoC extends D2 with App {
  println(s"Part 1 result: ${p1}")
  println(s"Part 2 result: ${p1+p2}")
}

trait D2 {
  var p1= 0
  var p2 = 0
  private val source = Source.fromFile(
    s"${System.getProperty("user.dir")}/src/main/scala/aoc/d2.txt"
  )

  for (line <- source.getLines()) {
    val numbers = line.split(" ").map(_.toInt)
   
    val isLevelOk = numbers
      .sliding(3, 1)
      .map { case Array(p, c, n) =>
        if (inBounds(p, c, n) && isContiguous(p, c, n)) true else false
      }
      .fold(true)(_ && _)
    if (isLevelOk)
      p1 += 1
      else if(trySingleBadLevel(numbers))
        p2+=1
  }
  def checkLevel(level: Array[Int]) =
    level
      .sliding(3, 1)
      .map { case Array(p, c, n) =>
        if (inBounds(p, c, n) && isContiguous(p, c, n)) true else false
      }
      .fold(true)(_ && _)

  def inBounds(p: Int, c: Int, n: Int): Boolean =
    return Math.abs(p - c) >= 1 && Math.abs(p - c) <= 3 && Math.abs(
      c - n
    ) >= 1 && Math.abs(c - n) <= 3
  def isContiguous(p: Int, c: Int, n: Int): Boolean =
    return (p > c && c > n) || (p < c && c < n)

  def trySingleBadLevel(level: Array[Int]): Boolean =
    return level.indices
      .map { i => checkLevel(level.slice(0, i) ++level.slice(i + 1, level.length))}
      .fold(false)(_ || _)}

