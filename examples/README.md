# Examples

## Design considerations

File references in vscode 

* @participant - a person or system that is involved in the process
* * e.g. @workspace to reference the workspace participant built into VSCode
* /command - a command involving the participant
* #variable - a variable that is used in the process

For agents we need a way to reference varaibles from the execution context to use as values.

## Context variables

{variable.attribute}

* variable is teh name of a variable within the current context
* attribute is the name of the attribute of the variable

## Script Syntax

Each script is a YAML file that contains one or more tasks. Each task as a separate YAML document in the file. The task
document contains the following fields:

```yaml
task:
  name: <task-name> # (1)
  template: # (1)
  model: # (2)
...
---
task:
  name: <task-name>
  template:
  model:
...
```

1. The unique name of the task. Task names are unique within the script.
2. The prompt template as text or the path and name of the file containing the prompt template.
3. The model to receive the prompt generated from applying the template to the source content. Model references includes
   the host, model name and version each separated by a colon. e.g. ollama:mistral:latest calls the configured ollama
   connection loading the mistral:latest model.

### Task structure

```yaml
task:
  name: <task-name> # (1)
  prompt: <prompt-name> # (2)
  source: # (3)
  sink: # (4)

```

1. The unique name of the task. Task names are unique within the script.
2. Prompt can either be the prompt template as text or the path and name of the file containing the prompt template.
3. Each task has a single task that provides values that are used to convert the prompt template into a prompt.
4. Where the response of executing the prompt against the model is to be stored.

### Supported sources

```yaml
task:
  source:
    files:
      - path: src/**/*.rs # (1)
```

1. The filesystem pattern of files that are to be provided to the prompt template for expansion. In the example all the
   files found (recursively) under the src directory with the .rs extension will be provided to the prompt template.

Template variables

| Variable         | Description                                                          |
|------------------|----------------------------------------------------------------------|
| {file.path}      | The path of the file                                                 |
| {file.name}      | The name of the file                                                 |
| {file.extension} | The extension of the file                                            |
| {file.content}   | The content of the file                                              |
| {file.type}      | The type or general name of the file. e.g. the .rs extension is Rust |

### Supported sinks

#### Filesystem

```yaml
  sink:
    type: filesystem # (1)
    path: /tmp/{file.path}/{file.name} # (2)
```

1. The type of sink. Currently only filesystem is supported.
2. The path to write the prompt response to. The path can contain template variables that are expanded using the source

Supported filesystem path variables

| Variable         | Description               |
|------------------|---------------------------|
| {file.path}      | The path of the file      |
| {file.name}      | The name of the file      |
| {file.extension} | The extension of the file |


## Environment

Environment variables are read from the current contenxt and can be used within a script.


## simple-prompt

Reads the script tasks from the `simple-prompt.yaml` file and runs them using the referenced prompt template and sources
referenced in the script.

```bash
$ cargo run -- --run simple-script.yaml
```
