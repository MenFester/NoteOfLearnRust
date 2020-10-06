# 学习笔记

## 7.1 简介

* 模式解构“Pattern Destructure”是Rust中一个重要且实用设计。Destructure意思是把原来的结构肢解为单独的、局部的、原始的部分
* ` let (head, center, tail) = tuple `赋值号左边的内容就是“模式”，赋值号右边的内容就是需要被“解构”的内容
* Rust中模式解构原则是：构造和解构遵循类似的语法
* Rust的“模式解构”功能不仅出现let语句中，还可以出现在match、if-let、while-let、函数调用、闭包调用等情景中
* match具有功能最强大的模式匹配

## 7.2 match

* 当一个类型有多种取值可能性的时候，特别适合使用match表达式
* 

## 7.3 if-let和while-let

## 7.4 函数和闭包参数做模式解构
