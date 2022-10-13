package com.enel.platform.rustlibrary.rust

class Hello
{
    init {
        System.loadLibrary("rusthello")
    }

    private external fun greeting(pattern: String): String?

    fun sayHello(to: String): String? {
        return greeting(to)
    }
}