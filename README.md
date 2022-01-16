# Simple Text Signer

Want to show that you didn't just fake that incredible Wordle guess?

Well... you still can, but you can at least prove you know the answer to the day's puzzle.

This is a simple tool that produces a message with a SHA256 hash that signs the text with a key phrase. 

## Example

This shows how to sign text from the command line. 
```sh
Dave$ target/debug/wordle-hash -k "faker" text 'Wordle 210 3/6*
> 
> ⬛⬛⬛🟨⬛
> 🟩🟩🟨🟨⬛
> 🟩🟩🟩🟩🟩'
Wordle 210 3/6*

⬛⬛⬛🟨⬛
🟩🟩🟨🟨⬛
🟩🟩🟩🟩🟩
sha256:pNyGjwj76Lma0e2Fi9wM8KdGc9YUar4GAZQQXDKDMt0=
```

And then you can take the "signed" text, and if you know the key phrase, you can verify they signed it with the right word.
```sh
Dave$ target/debug/wordle-hash -k "faker" verify 'Wordle 210 3/6*
> 
> ⬛⬛⬛🟨⬛
> 🟩🟩🟨🟨⬛
> 🟩🟩🟩🟩🟩
> sha256:pNyGjwj76Lma0e2Fi9wM8KdGc9YUar4GAZQQXDKDMt0=
> '
Verified!
```

You can also use a file as an input.
