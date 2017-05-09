package main

import (
	"github.com/loadoff/excl"
)

func main() {
	w, _ := excl.Create()
	s, _ := w.OpenSheet("Sheet1")
	s.Close()
	w.Save("./new.xlsx")

	// Excelファイルを読み込み
	ww, _ := excl.Open("./new.xlsx")
	// シートを開く
	ss, _ := ww.OpenSheet("Sheet1")
	// 一行目を取得
	r := ss.GetRow(1)
	// 1列目のセルを取得
	c := r.GetCell(1)
	// セルに10を出力
	c.SetNumber("10")
	// 2列目のセルにABCDEという文字列を出力
	c = r.SetString("ABCDE", 2)
	// シートを閉じる
	ss.Close()
	// 保存
	ww.Save("./new.xlsx")
}
