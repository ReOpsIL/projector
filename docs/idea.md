
### **Prompt: LLM-Powered Dynamic Project Definition Wizard**

I want to build an intelligent, interactive wizard driven by a Large Language Model (LLM) that guides users through a context-sensitive sequence of questions, ultimately generating a comprehensive and customized **project definition for an LLM-based application or system**.

#### ✅ **Core Workflow**

* The wizard begins by asking the user a series of dynamically generated questions, crafted by the LLM based on:

    * User-provided starting hints (optional)
    * The accumulated context from previous answers
    * Domain-specific knowledge (if a domain is selected)
* Questions will vary in type:

    * Multiple-choice (1–4 or more options)
    * Yes/No
    * Rating scales (e.g., 1–5 importance/priority)
    * Short free-text entries (validated by the LLM or used for context enrichment)

#### 🔁 **Adaptive Question Flow**

* Questions are **context-aware**: each answer influences the next question.
* The wizard runs for **N iterations**, where *N* is configurable or user-controlled.
* Option to use **branching logic**: diverging question paths depending on previous answers.
* A **"back" feature** allows users to revise earlier answers and reflow the logic tree accordingly.

#### 🧠 **LLM-Generated Project Output**

At the end of the session, the wizard will automatically generate a detailed project definition document that includes:

* Project name and short summary
* Usecas and goals (with examples or scenarios)
* Target user profile(s)
* Required inputs and expected outputs
* Functional components/modules
* Prompt engineering strategy
* Dataset needs and sources (including synthetic data generation if required)
* Evaluation metrics and success criteria
* Scalability and deployment recommendations (e.g., cloud, edge, embedded)
* Optional ethical and bias considerations section

#### ⚙️ **Additional Features**

* **Predefined Templates/Presets**: Start with industry-specific wizards (e.g., legal assistant, medical chatbot, code explainer).
* **Persona Modes**: The wizard can simulate questioning from the point of view of a:
    * Product Manager
    * LLM Architect
    * UX Designer
    * Compliance Officer
  
* **Session Export Options**:
    * Markdown summary

* **Confidence Scoring**: Each section of the final spec may include a confidence level based on the specificity and completeness of user answers.




