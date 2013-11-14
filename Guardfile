guard :shell do
  watch(/\.rs/) do |m|
    title = 'Compile'
    msg = `make`
    status = ($?.success? && :success) || :failed

    n msg, title, status
    "-> #{msg}"
  end
end
