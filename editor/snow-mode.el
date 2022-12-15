;; Copyright (C) 2021 Alexey Kutepov <reximkut@gmail.com>

;; Author: Alexey Kutepov <reximkut@gmail.com>
;; URL: https://github.com/tsoding/porth

;; Permission is hereby granted, free of charge, to any person
;; obtaining a copy of this software and associated documentation
;; files (the "Software"), to deal in the Software without
;; restriction, including without limitation the rights to use, copy,
;; modify, merge, publish, distribute, sublicense, and/or sell copies
;; of the Software, and to permit persons to whom the Software is
;; furnished to do so, subject to the following conditions:

;; The above copyright notice and this permission notice shall be
;; included in all copies or substantial portions of the Software.

;; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
;; EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
;; MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
;; NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
;; BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
;; ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
;; CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
;; SOFTWARE.

(defconst snow-mode-syntax-table
  (with-syntax-table (copy-syntax-table)
    ;; C/C++/Rust style comments
    (modify-syntax-entry ?/ ". 124b")
    (modify-syntax-entry ?* ". 23")
    (modify-syntax-entry ?\n "> b")
    ;; Chars are the same as strings
    (modify-syntax-entry ?' "\"")
    (syntax-table))
  "Syntax table for `snow-mode'.")

(eval-and-compile
  (defconst snow-keywords
    '("type" "fn" "if" "then" "else" "true" "false" "let" "in" "use")))


(defconst snow-highlights
  `((,(regexp-opt snow-keywords 'symbols) . font-lock-keyword-face)))

;;;###autoload
(define-derived-mode snow-mode prog-mode "snow"
  "Major Mode for editing Snow source code."
  :syntax-table snow-mode-syntax-table
  (setq font-lock-defaults '(snow-highlights))
  (setq-local comment-start "// "))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.snow\\'" . snow-mode))

(provide 'snow-mode)

;;; snow-mode.el ends here
