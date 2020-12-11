import * as fs from 'fs'

function readInputData() : string[] {
  const data: string = fs.readFileSync('input.txt', 'utf8')
  var lines: string[] = data.split('\n')

  return lines
}

function count(word: string, letter: string) : number {
  let count : number = 0
  for (let l of word) {
    if (l === letter) {
      count++
    }
  }

  return count
}

function findSolutionPart1(lines: string[]) {
  let valid : number = 0
  for (let l of lines) {
    let m = l.match(/(\d+)-(\d+) ([a-zA-Z]): (.*)/);
    let minOccurs : number = Number(m[1])
    let maxOccurs : number = Number(m[2])
    let letter = m[3]
    let password = m[4]

    let cnt = count(password, letter)
    //console.log(`The letter ${letter} must occur between ${minOccurs} and ${maxOccurs} times.  The password is ${password}.  It occurs ${cnt} times.`)
    if ((cnt >= minOccurs) && (cnt <= maxOccurs)) {
      valid++
    }
  }

  console.log(`Total valid ${valid}`)
}

function findSolutionPart2(lines: string[]) {
  let valid : number = 0
  for (let l of lines) {
    let m = l.match(/(\d+)-(\d+) ([a-zA-Z]): (.*)/);
    let pos1 : number = Number(m[1])
    let pos2 : number = Number(m[2])
    let letter = m[3]
    let password = m[4]

    pos1--; pos2--

    let match : number = 0
    if (password.charAt(pos1) === letter) {
      match++
    }

    if (password.charAt(pos2) === letter) {
      match++
    }

    if (match === 1) {
      valid++
    }
  }

  console.log(`Total valid ${valid}`)
}

let lines: string[] = readInputData()
findSolutionPart1(lines)
findSolutionPart2(lines)
