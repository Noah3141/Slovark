import mysql.connector
class LemmatizerDb:
    config = {
        'user': 'root',
        'password': 'adH65GF3hH2B1BDdeCGBe2cCcBHfB4HC',
        'host': 'roundhouse.proxy.rlwy.net',
        'database': 'railway',
        'raise_on_warnings': True,
        'port': "35412",
    }

    @classmethod
    def get_rows(cls,):
        try: 
            conn = mysql.connector.connect(**cls.config)
            cursor = conn.cursor()
            cursor.execute("SELECT DISTINCT lemma FROM russian_words;")
            rows = cursor.fetchall()

            return rows

        except mysql.connector.Error as err:
            print(f"Error: {err}")

        finally:
            # Close database connection
            if 'conn' in locals():
                conn.close()
                print("MySQL connection closed.")