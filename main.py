

#
# Complete the 'palindromChecker' function below.
#
# The function is expected to return an INTEGER.
# The function accepts STRING initial as parameter.
#

def palindromChecker(initial : str ):
    # # Write your code here
    return int(  initial == initial[::-1] )



if __name__ == "__main__":
    # fptr = open(os.environ['OUTPUT_PATH'], 'w')

    initial = input()

    result = palindromChecker(initial)

    print(result)

    # fptr.write(str(result) + '\n')

    # fptr.close()