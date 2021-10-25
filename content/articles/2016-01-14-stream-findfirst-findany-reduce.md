---
title: "Beware Of findFirst() And findAny()"
tags: [java-8, streams]
date: 2016-01-14
slug: java-stream-findfirst-findany-reduce
description: "`Stream.findFirst()` and `findAny()` work with any number of elements in the stream. Make sure to `reduce(toOnlyElement())` if there should be at most one."
intro: "When using `Stream.findFirst()` or `findAny()`, you will often assume that there is at most one element left in the stream. But neither tests that assumption so maybe you should use a different approach."
searchKeywords: "stream findFirst findAny"
featuredImage: stream-findfirst-findany-reduce
---

Lorem ipsum dolor sit amet, consectetur adipisici elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat.
Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint obcaecat cupiditat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.