import uniffi_trait as t

class TestMyTraitPython(t.MyTrait):
  def my_trait(self):
    return "banana"    

if __name__ == '__main__':
    a = TestMyTraitPython()
    print(t.ciao(a))
