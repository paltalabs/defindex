using System;
using Xunit;

namespace DotNetSdkTests
{
  public class UnitTest1
  {
    [Fact]
    public void Test1()
    {
      // Arrange
      int expected = 5;
      int actual = 2 + 3;

      // Act & Assert
      Assert.Equal(expected, actual);
    }
  }
}