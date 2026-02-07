# Message Parsing Documentation

Documentation for how ch-cli parses and stores user messages.

## Contents

- [Message Parsing](./MESSAGE_PARSING.md) - Complete message parsing system documentation

## Overview

The message parsing system handles:
- Distinguishing text input from file/folder references
- Tracking file and folder paths with context
- Maintaining conversation history
- Smart whitespace and path handling
- Visual highlighting in the UI

Messages are parsed into segments:
- **Text** - Plain user input
- **FileReference** - References to files in the picker
- **FolderReference** - References to folders in the picker

This allows ch-cli to maintain rich context about what files and folders the user is working with.
