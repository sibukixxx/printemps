<?php

use Illuminate\Http\Request;

Route::group(['middleware' => 'api'], function() {
    Route::get('articles',  function() {
        $articles = Article::all()->take(5);
        return $articles;
    });
});
