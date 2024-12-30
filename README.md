# AWS Quiz

Basic Slint desktop application built to study for an AWS AI Practitioner exam, but it can be used for anything that can use a four answer prompt. This was a one and done for me so I'm not supporting this at all, it was mostly to practice for an exam and to practice some Slint/Rust.

# Usage
Construct a JSON question series using the format:
```json
[
   {
      "question": "The question",
      "answers": [
         "answer 1",
         "answer 2",
         "answer 3",
         "answer 4"
      ],
      "correct_answer": 2,
      "explanation": [
         "answer 1 explanation",
         "answer 2 explanation",
         "answer 3 explanation",
         "answer 4 explanation"
      ]
   }
]
```
and insert into the root directory. Change the name of the file to be either questions.json or change the file name inside the source code to match whichever you prefer.
