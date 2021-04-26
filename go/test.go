package main

import (
  "fmt"
  "log"
  "bytes"
  "io/ioutil"
  "github.com/PuerkitoBio/goquery"
)

func ExampleScrape() {

    // Read entire file content, giving us little control but
    // making it very simple. No need to close the file.
    content, err := ioutil.ReadFile("../rust/src/test.html")
    if err != nil {
        log.Fatal(err)
    }

  // Load the HTML document
  doc, err := goquery.NewDocumentFromReader(bytes.NewReader(content))
  if err != nil {
    log.Fatal(err)
  }

  // Find the review items
  doc.Find("body").Each(func(i int, s *goquery.Selection) {
    // For each item found, get the band and title
//     band := s.Find("a").Text()
//     title := s.Find("i").Text()
//     fmt.Printf("Review %d: %s - %s\n", i, band, title)
  })
}

func main() {
  ExampleScrape()
}