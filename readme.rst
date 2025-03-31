##############
Nerd Dictation for Bazzite
##############

*Offline Speech to Text for Desktop Linux.* - See `demo video <https://www.youtube.com/watch?v=T7sR-4DFhpQ>`__.

This is a utility that provides simple access speech to text for using in Linux
without being tied to a desktop environment, using the excellent `VOSK-API <https://github.com/alphacep/vosk-api>`__.

Simple
   This is a single file Python script with minimal dependencies.
Hackable
   User configuration lets you manipulate text using Python string operations.
Zero Overhead
   As this relies on manual activation there are no background processes.

Dictation is accessed manually with begin/end commands.

Install
=======

For Bazzite users:

.. code-block:: sh

   git clone https://github.com/RadiantOblivion/nerd-dictation-bazzite.git
   cd nerd-dictation-bazzite
   chmod +x install-bazzite
   ./install-bazzite

Usage
=====

For Bazzite users, use the following command to **toggle** dictation:

.. code-block:: sh

   pipx-nerd-dictation

Features
========

Specific features include:

Numbers as Digits
   Optional conversion from numbers to digits.

   So ``Three million five hundred and sixty second`` becomes ``3,000,562nd``.

   A series of numbers (such as reciting a phone number) is also supported.

   So ``Two four six eight`` becomes ``2,468``.

Time Out
   Optionally end speech to text early when no speech is detected for a given number of seconds.
   (without an explicit call to ``end`` which is otherwise required).

Output Type
   Output can simulate keystroke events (default) or simply print to the standard output.

User Configuration Script
   User configuration is just a Python script which can be used to manipulate text using Python's full feature set.

Suspend/Resume
   Initial load time can be an issue for users on slower systems or with some of the larger language-models,
   in this case suspend/resume can be useful.
   While suspended all data is kept in memory and the process is stopped.
   Audio recording is stopped and restarted on resume.

Configuration
=============

This is an example of a trivial configuration file which simply makes the input text uppercase.

.. code-block:: python

   # ~/.config/nerd-dictation/nerd-dictation.py
   def nerd_dictation_process(text):
       return text.upper()

Paths
=====

Local Configuration
   ``~/.config/nerd-dictation/nerd-dictation.py``
Language Model
   ``~/.config/nerd-dictation/model``

Limitations
===========

- Text from VOSK is all lower-case,
  while the user configuration can be used to set the case of common words like ``I`` this isn't very convenient
  (see the example configuration for details).

- For some users the delay in start up may be noticeable on systems with slower hard disks
  especially when running for the 1st time (a cold start).

  This is a limitation with the choice not to use a service that runs in the background.
  Recording begins before any the speech-to-text components are loaded to mitigate this problem.
