package com.urv.codetranslator

import android.os.Bundle
import android.widget.*
import androidx.appcompat.app.AppCompatActivity
import com.google.android.material.textfield.TextInputEditText

class MainActivity : AppCompatActivity() {
    
    private lateinit var sourceLanguageSpinner: Spinner
    private lateinit var targetLanguageSpinner: Spinner
    private lateinit var inputCodeEditText: TextInputEditText
    private lateinit var outputCodeTextView: TextView
    private lateinit var translateButton: Button
    
    companion object {
        init {
            System.loadLibrary("code_translator_android")
        }
    }
    
    // Native methods declaration
    private external fun translateCode(
        sourceCode: String,
        sourceLanguage: String,
        targetLanguage: String
    ): String
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        
        sourceLanguageSpinner = findViewById(R.id.sourceLanguageSpinner)
        targetLanguageSpinner = findViewById(R.id.targetLanguageSpinner)
        inputCodeEditText = findViewById(R.id.inputCodeEditText)
        outputCodeTextView = findViewById(R.id.outputCodeTextView)
        translateButton = findViewById(R.id.translateButton)
        
        val languages = arrayOf(
            "C", "C++", "C#", "Java", "JavaScript", "TypeScript", "Python",
            "Ruby", "Swift", "Kotlin", "Go", "Rust", "PHP", "Scala", "Dart",
            "Haskell", "Elixir", "F#", "R", "MATLAB", "D", "SQL",
            "Assembly x86", "Assembly x64", "Assembly ARM", "WebAssembly",
            "Pseudocódigo", "Pseudocódigo C", "Pseudocódigo Java", "Pseudocódigo Python"
        )
        
        val adapter = ArrayAdapter(this, android.R.layout.simple_spinner_item, languages)
        adapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item)
        
        sourceLanguageSpinner.adapter = adapter
        targetLanguageSpinner.adapter = adapter
        targetLanguageSpinner.setSelection(languages.indexOf("Python"))
        
        translateButton.setOnClickListener {
            val sourceCode = inputCodeEditText.text.toString()
            val sourceLang = sourceLanguageSpinner.selectedItem.toString()
            val targetLang = targetLanguageSpinner.selectedItem.toString()
            
            if (sourceCode.isNotBlank()) {
                try {
                    val result = translateCode(sourceCode, sourceLang, targetLang)
                    outputCodeTextView.text = result
                } catch (e: Exception) {
                    outputCodeTextView.text = "Error: ${e.message}"
                }
            } else {
                Toast.makeText(this, "Introduce código a traducir", Toast.LENGTH_SHORT).show()
            }
        }
    }
}
